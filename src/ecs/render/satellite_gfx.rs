use super::super::physics::{TransformComponent, Orbit};


use specs::{
    System, 
    Component, 
    VecStorage, 
    ReadStorage, 
    Join,
};

use macroquad::{
    models::draw_sphere,
    color::{Color, colors::BLUE},
    math::{vec3, Vec3, Quat},
};

use crate::extend_microquad_gfx::draw_ellipse;


pub enum ModelType {
    Sphere { radius: f32 },
    Icon,
}



///Component that holds graphical information for an orbital body.
///Note that mass should be in Kg and Vectors are in meters.
#[derive(Component)]
#[storage(VecStorage)]
pub struct SatelliteGFX {
    //2d graphic to display if we are far enough away from the camera    
    //icon: 

    //aka sphere for planets, random convex shape for asteroids/etc, or model for ships/stations
    pub model: ModelType,

    //Texture to apply to the above model.
    //texture: Option<?>, 
    
    //plain color to apply to sphere in lieu of a texture.
    pub color: Color, //plain color to apply to sphre in lieu of a texture.
}


///System that handles advancing the position of all Satellites
pub struct OrbitDisplaySystem;
impl<'a> System<'a> for OrbitDisplaySystem {

    type SystemData = (ReadStorage<'a, TransformComponent>,
                       ReadStorage<'a, Orbit>,
                       ReadStorage<'a, SatelliteGFX>);
                       

    fn run(&mut self, (transforms, orbits, graphics): Self::SystemData) {

        for (transform, orbit, graphic) in 
            ((&transforms).maybe(), (&orbits).maybe(), &graphics).join() {
            
            //we have to figure out position depending on the avalible component
            let mut position = None;
            
            //if we have the tranform component then the position is easy to figure out
            if let Some(transform) = transform {
                //macroquad uses a vector type s we have to make one for this
                position = Some( 
                    vec3( (transform.x*10.) as f32,
                          (transform.y*10.) as f32,
                          (transform.z*10.) as f32) 
                );
            }
            
            //if we have an orbit we need to calculate cartisian location
            if let Some(orbit) = orbit {
                //first figure out positon of the satellite

                //> create an x identity vector
                let mut position2 = Vec3::X;
                //> lengthen this by the current orbit height.
                //  Note it is in KM and we need to make it waaay smaller
                position2 *= ( orbit.height() / 50000000. ) as f32;
                let rotation = Quat::from_rotation_y(orbit.inclination as f32).mul_quat(
                    Quat::from_rotation_z(orbit.longitude as f32 + orbit.true_anomaly as f32)
                );
                
                //rotate the height vector so we are in the right location
                position = Some( rotation.mul_vec3(position2) );
                
                //next we draw out the satellite's orbit
                draw_ellipse(orbit.semi_major / 50000000., orbit.eccentricity, 
                              orbit.inclination, orbit.longitude, BLUE);
                
            }

            //only draw the satellite if position was set. 
            if let Some(position) = position {
                //Draw the correct shape and fill for each entity.
                //match (&graphic.model, &graphic.texture) {
                match &graphic.model {
                    ModelType::Sphere { radius } =>
                        draw_sphere(position, *radius, None, graphic.color),
                    _ => {}
                }            
            }
        }
    
    }
}
