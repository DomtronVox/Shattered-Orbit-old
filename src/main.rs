mod ecs;
use ecs::ECSManager;

use macroquad::prelude::*;

mod ui_state;
use ui_state::{UIStateMachine, ControlData};

#[derive(Default)]
pub struct DeltaTime(f64);

pub struct Simulation {
    pub ecs: ECSManager,
}


#[macroquad::main("Shattered Orbit")]
async fn main() {  
    
    let mut ui_statemachine = UIStateMachine::new();
    
    let mut simulation = Simulation {
        ecs: ECSManager::new(),
    };

    while ui_statemachine.running {

        //update model
        simulation.ecs.update(4000.);

        //setup what's needed for drawing
        clear_background(BLACK);

        //to draw the ui we need to reset the camera incase anything else changed it
        set_default_camera();
        ui_statemachine.update(&mut simulation); 
        
        //draw everything
        next_frame().await   
    }

    //TODO run simulation shutdown here.

}
