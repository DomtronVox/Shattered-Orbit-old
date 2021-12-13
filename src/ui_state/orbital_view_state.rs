use super::{UIState, StateEvent, UIStateMachine, view_switcher};
use crate::Simulation;



use macroquad::{
    math::vec2,
    window::{screen_width, screen_height},
    ui::{hash, root_ui, widgets},
};




pub struct OrbitalViewState {}

impl OrbitalViewState {
    pub fn new() -> OrbitalViewState {
        OrbitalViewState {}
    }
}

impl UIState for OrbitalViewState {

    fn ui_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
 
        //setup View Switcher and containing window
        let view_switcher_id = hash!();
        let view_switcher_pos = vec2(0., screen_height() - 40.);
        
        root_ui().window(view_switcher_id, view_switcher_pos, vec2(screen_width(), 40.),
            |ui| { 
                //make sure to update window position so it says in the right place on resize
                ui.move_window(view_switcher_id, view_switcher_pos);

                view_switcher(1, state_machine, ui, vec2(screen_width()-5., 35.));
            }
        );
    }

    
    fn state_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        
    }
}
