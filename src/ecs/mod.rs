use crate::DeltaTime;

/*mod control;
use control::{build_control_dispatcher, PlayerControlComponent};
*/

mod physics;
use physics::{build_physics_dispatcher, TransformComponent, Orbit};

mod render;
use render::{build_render_orbits_dispatcher, ModelType, SatelliteGFX};

/*mod mechanics;
use mechanics::{build_mechanics_dispatcher, ChunkLoadingComponent};
*/

use specs::{World, WorldExt, Builder, Dispatcher};

/// Holds the World and manages dispatching the various systems for it
//#[derive(Default)]
pub struct ECSManager {
    pub world: World,
    
    pub sim_dispatchers: Vec< Dispatcher<'static, 'static> >,
    pub orbits_dispatcher: Dispatcher<'static, 'static>,
}

impl ECSManager {

    /// Sets up the basic world. 
    pub fn new() -> Self {
        let mut world = World::new();
        
        //setup world values
        world.insert(DeltaTime::default());
        
        
        //setup the system dispatcher groups
        let mut orbits_dispatcher = build_render_orbits_dispatcher();
        orbits_dispatcher.setup(&mut world);
        
        let mut sim_dispatchers = vec!();
        sim_dispatchers.push(build_physics_dispatcher());
        
        //use specs setup stage to auto register components
        for dispatcher in &mut sim_dispatchers { 
            dispatcher.setup(&mut world);
        }
        
        
        //dev function to create test entities
        //create_test_entities(&mut world);
        
        
        ECSManager {
            world,
            sim_dispatchers,
            orbits_dispatcher,
        }
    }
    
    /// Runs the dispatchers related to game simulation.
    pub fn update(&mut self, dt: f64) {
    
        //set time change
        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt);
        }
        
        
        for dispatcher in &mut self.sim_dispatchers {
            dispatcher.dispatch(&self.world);
        }
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
pub fn create_test_entities(world: &mut World) {
    //Earth
    world.create_entity()
    .with(TransformComponent { x: 0., y: 0., z:0., angle: 0.})
    .with(SatelliteGFX { 
        model: ModelType::Sphere{ radius: 1. }, color: Color::new(0.,150.,255., 290.) }
    )
    .build();

    //moon
    world.create_entity()
    .with(Orbit::new(5.9722e24, 384748000., 0.0549006, 0.02693043, 0., 0.))
    .with(SatelliteGFX { 
        model: ModelType::Sphere{ radius: 0.5 }, color: Color::new(255.,255.,255., 1.) }
    )
    .build();
    
    //moon
    world.create_entity()
    .with(Orbit::new(5.9722e24, 184748000., 0.6549006, 0.92693043, 0., 0.))
    .with(SatelliteGFX { 
        model: ModelType::Sphere{ radius: 0.1 }, color: Color::new(255.,0.,0., 1.) }
    )
    .build();
}
