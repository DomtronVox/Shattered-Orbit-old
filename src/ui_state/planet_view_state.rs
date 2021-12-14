use super::{UIState, StateEvent, UIStateMachine, view_switcher};
use crate::Simulation;



use macroquad::{
    math::vec2,
    window::{screen_width, screen_height},
    ui::{hash, root_ui, widgets},
};




pub struct PlanetViewState {}

impl PlanetViewState {
    pub fn new() -> PlanetViewState {
        PlanetViewState {}
    }
}

impl UIState for PlanetViewState {

    fn world_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        
    }

    fn ui_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
 
        //setup View Switcher and containing window
        let view_switcher_id = hash!();
        let view_switcher_pos = vec2(0., screen_height() - 40.);
        
        root_ui().window(view_switcher_id, view_switcher_pos, vec2(screen_width(), 40.),
            |ui| { 
                //make sure to update window position so it says in the right place on resize
                ui.move_window(view_switcher_id, view_switcher_pos);

                view_switcher(2, state_machine, ui, vec2(screen_width()-5., 35.));
            }
        );
    }

    
    
}
