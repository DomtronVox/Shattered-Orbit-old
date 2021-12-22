//Information: http://www.braeunig.us/space/orbmech.htm

use std::f64::consts::PI;

use rayon::prelude::*;
use specs::{
    System, 
    Component, 
    VecStorage, 
    Read,
    WriteStorage, 
    Join,
    ParJoin,
};




///Component that holds orbit info
#[derive(Component)]
#[storage(VecStorage)]
pub struct Orbit {
    //acceleration due to primary body
    pub mu: f64,
    //Semi-major axis aka size of orbit
    pub semi_major: f64,
    //Eccentricity aka shape of orbit
    pub eccentricity: f64,
    //Tilt of orbit from ref plane (x-y plane)
    pub inclination: f64,
    //swivel
    pub longitude: f64,
    //angle from acending node and periapsis
    //pub argument_of_periapsis: f64,
    //position in orbit
    pub true_anomaly: f64,
}

impl Orbit {

        //Construct a new Orbit components
        // Note the mass is not the satellite's mass but the mass of what it's orbiting
        pub fn new(primary_mass: f64, semi_major: f64, eccentricity: f64, inclination: f64,
                    longitude: f64, true_anomaly: f64) 
          -> Self {
          
            let mu = Orbit::calc_mu(primary_mass);
            
            Orbit {
                mu,                      // N-m2/kg2 * kg
                semi_major,              // m
                eccentricity,            // unitless
                inclination,             // radians
                longitude,               // radians
                //argument_of_periapsis,   // radians
                true_anomaly,            // radians
            }
        }
        

        //Calculates the Mu (Grav-const * mass) of an orbit
        fn calc_mu(mass: f64) -> f64 {
            6.674e-11 * mass //N-m2/kg2 * kg
        }
        
        //calculate eccentric anomaly or E = atan( sqrt( (1-e) / (1+e) ) * tan( v / 2) )
        fn eccentric_anomaly(&self) -> f64 {
            //arctan only functions between -PI and PI so adjust if over PI
            let true_anomaly =
                if self.true_anomaly <= PI { self.true_anomaly }
                else                       { self.true_anomaly - (2. * PI) };
            
            (
                ( ( 1. - self.eccentricity ) / ( 1. + self.eccentricity ) ).sqrt() *
                ( true_anomaly / 2. ).tan()
            ).atan() * 2.
        }
        
        // calculate mean anomaly or M = E - e * cos(E)
        fn mean_anomaly(&self) -> f64 {
            let ea = self.eccentric_anomaly();
            ea - self.eccentricity * ea.sin()
        }

        //calculate mean motion or n = sqrt( mu / a^3 )
        fn mean_motion(&self) -> f64 {
            ( self.mu / self.semi_major.powi(3) ).sqrt()
        }
        
        //calculate an approximation of true anomaly from a given mean anomaly
        // Note, needs very small eccentricity since error is e^3
        fn true_anomaly_by_approximation(&self, mean_anomaly: f64) -> f64 {
            mean_anomaly + 2. * self.eccentricity * mean_anomaly.sin() +
                1.25 * self.eccentricity.powi(2) * (2. * mean_anomaly).sin()
        }
        
        //calculate true anomaly via iteration
        fn true_anomaly_by_iteration(&self, mean_anomaly: f64) -> f64 {
        
            //we need to calculate eccentric anomaly by iteration
            let mut eccentric_anomaly = mean_anomaly; //start our guess with mean anomaly
            let mut test_mean_anomaly = 0.; //result from our guesses
            
            //> iterate at most 30 times
            for iteration in (0..29) {
                //we try our guess in the formula
                test_mean_anomaly = 
                    eccentric_anomaly - self.eccentricity * eccentric_anomaly.sin();
                
                //we check which direction to alter our guess and alter it by the difference
                //Note: had to hack in a round to decimal places function (defined at top) 
                //  because rust doesn't have one
                if        test_mean_anomaly > mean_anomaly { 
                    eccentric_anomaly -= test_mean_anomaly-mean_anomaly;
                    
                } else if test_mean_anomaly < mean_anomaly {
                    eccentric_anomaly += mean_anomaly-test_mean_anomaly;
                
                //end the loop when the two values are equal    
                } else { break; }
                
            }
            
            //arctan only functions between -PI and PI so adjust if over PI
            let eccentric_anomaly =
                if eccentric_anomaly <= PI { eccentric_anomaly }
                else                       { eccentric_anomaly - (2. * PI) };
            
            //Now we can use the eccentric anomaly to calculate true anomaly
            //( (eccentric_anomaly.cos() - self.eccentricity) / 
            //    (1. - self.eccentricity * eccentric_anomaly.cos()) ).acos()
            (
                ( eccentric_anomaly / 2. ).tan() / 
                ( ( 1. - self.eccentricity ) / ( 1.+ self.eccentricity ) ).sqrt()
            ).atan() * 2.
        }
                
        //calculate orbital position after a span of time of new_M = n * t + cur_M
        pub fn position_after_time(&self, time_span: f64) -> f64 {
        
            let new_m = self.mean_motion() * time_span + self.mean_anomaly();
            
            //approximation formula is good for low eccentricity since it's error is e^3
            // otherwise we have to run it by iteration
            let mut new_ta = 
                if self.eccentricity < 0.09 {
                    self.true_anomaly_by_approximation( new_m )
                } else {
                    self.true_anomaly_by_iteration( new_m )
                };
                
            //If we are over 2PI we want to adjust back down so we always stay between 0 and 2PI
            if self.true_anomaly >= 2. * PI { new_ta -= 2. * PI; }
            
            new_ta
        }
        
        //calculate height from the primary body or a ( 1 - e^2 ) / 1 + e cos(v)
        pub fn height(&self) -> f64 {
            (self.semi_major * (1. - self.eccentricity.powi(2)) ) / ( 1. + self.eccentricity * self.true_anomaly.cos() )
        }
        
        //calculate flight-path angle or atan( e sin(v) / 1 + e cos(v) )
        pub fn flight_path_angle(&self) -> f64 {
            ( (self.eccentricity * self.true_anomaly.sin() ) / 
                ( 1. + self.eccentricity *  self.true_anomaly.cos() ) ).atan()
        }
        
        //Calculate current velocity or sqrt( mu ( 2/r - 1/a ) )
        pub fn velocity(&self) -> f64 {
            ( self.mu * ( 2. / self.height() - 1. / self.semi_major ) ).sqrt()
        }
}


///System that handles advancing the position of all Satellites
pub struct OrbitMotionSystem {

}


impl<'a> System<'a> for OrbitMotionSystem {

    type SystemData = (Read<'a, crate::DeltaTime>,
                       WriteStorage<'a, Orbit>);
                       

    fn run(&mut self, (delta, mut orbit_data): Self::SystemData) {
        let dt = delta.0;

        //Process all orbits in parallel advancing their position base on time passed
        (&mut orbit_data).par_join().for_each(|orbit| {
            orbit.true_anomaly = orbit.position_after_time(dt);
        });
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    ///Test orbital Math
    fn orbital_math() {
    
        //see https://www.braeunig.us/space/problem.htm#4.13
        // 5.9722e24 mass of earth
        // 7,500 km semi-major axis, .1 eccentricity, 0.52360 rad true anomoly
        // inclination and longetude does not matter.
        let mut orbit = Orbit::new(5.9722e24, 7500000., 0.1, 0., 0., 0.52360);
        
        //test mu value
        assert_eq!(orbit.mu, 3.98584628e14);
        
        //test eccentric anomaly
        assert_eq!(orbit.eccentric_anomaly(orbit.true_anomaly), 0.4755689321854411);
        
        //test mean anomaly
        assert_eq!(orbit.mean_anomaly(orbit.true_anomaly), 0.42978450014676683);
        
        //test mean motion
        assert_eq!(orbit.mean_motion(), 0.000972004728535976);
        
        //using problem 3.14 data
        orbit = Orbit::new(5.9722e24, 7500000., 0.1, 0., 0., 1.57080);
        
        //test calculating a new true anomaly after 20 minutes, 1200 seconds
        assert_eq!(orbit.position_after_time(1200.), 2.639449246489066);
    }
}
