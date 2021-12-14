/*mod control;
use control::{build_control_dispatcher, PlayerControlComponent};
*/

mod physics;
use physics::{TransformComponent};

mod render;
use render::{build_render_orbits_dispatcher, SatelliteGFX};

/*mod mechanics;
use mechanics::{build_mechanics_dispatcher, ChunkLoadingComponent};
*/

use specs::{World, WorldExt, Builder, Dispatcher};

/// Holds the World and manages dispatching the various systems for it
//#[derive(Default)]
pub struct ECSManager {
    pub world: World,
    
    //pub sim_dispatchers: Vec< Dispatcher<'static, 'static> >,
    pub orbits_dispatcher: Dispatcher<'static, 'static>,
}

impl ECSManager {

    /// Sets up the basic world. 
    pub fn new() -> Self {
        let mut world = World::new();
        
        //setup world values
        //world.insert();
        
        
        //setup the system dispatcher groups
        let mut orbits_dispatcher = build_render_orbits_dispatcher();
        orbits_dispatcher.setup(&mut world);
        
        //let mut sim_dispatchers = vec!();
        //sim_dispatchers.push(build_mechanics_dispatcher());
        
        //use specs setup stage to auto register components
        //for dispatcher in &mut sim_dispatchers { 
        //    dispatcher.setup(&mut world);
        //}
        
        
        //dev function to create test entities
        create_test_entities(&mut world);
        
        
        ECSManager {
            world,
            //sim_dispatchers,
            orbits_dispatcher,
        }
    }
    
    /// Runs the dispatchers related to game simulation.
    pub fn update(&mut self) {
        //for dispatcher in &mut self.sim_dispatchers {
       //     dispatcher.dispatch(&self.world);
        //}
    }
    
    //run systems that render orbital satelites
    pub fn render_orbits(&mut self) {
        self.orbits_dispatcher.dispatch(&self.world);
    }
    
    //run systems to render planetary objects
    pub fn render_planet(&mut self) {
    
    }
    
    //run systems to render the station
    pub fn render_station(&mut self) {
    
    }
}


///Dev function for creating entities until we impl data loading
use macroquad::color::Color;
fn create_test_entities(world: &mut World) {
    //Earth
    world.create_entity()
    .with(TransformComponent { x: 0., y: 0., z:0., angle: 0.})
    .with(SatelliteGFX { color: Color::new(0.,150.,255., 0.5) })
    .build();

    
}
