use super::{UIState, StateEvent, UIStateMachine, view_switcher};
use crate::Simulation;



use macroquad::{
    math::vec2,
    window::{screen_width, screen_height},
    ui::{hash, root_ui, widgets},
};




pub struct MissionViewState {}

impl MissionViewState {
    pub fn new() -> MissionViewState {
        MissionViewState {}
    }
}

impl UIState for MissionViewState {

    fn ui_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {

        //calculate size and position of side panel
        let left_menu_id = hash!();
        let left_size = vec2(screen_width() / 2., screen_height() - 38.);
        let left_pos = vec2( 0., 0.);
        
        //Setup a window that will hold side menu options.
        root_ui().window(left_menu_id, left_pos, left_size,         
            |ui| {
                
                
            },
        );

        //calculate size and position of side panel
        let right_menu_id = hash!();
        let right_size = vec2(screen_width() / 2., screen_height() - 38.);
        let right_pos = vec2( screen_width() - (right_size.x) , 0. );
        
        //Setup a window that will hold side menu options.
        root_ui().window(right_menu_id, right_pos, right_size,         
            |ui| {
                //make sure to update window position so it says in the right place on resize
                ui.move_window(right_menu_id, right_pos);
                
                
            },
        );


        //setup View Switcher and containing window
        let view_switcher_id = hash!();
        let view_switcher_pos = vec2(0., screen_height() - 40.);
        
        root_ui().window(view_switcher_id, view_switcher_pos, vec2(screen_width(), 40.),
            |ui| { 
                //make sure to update window position so it says in the right place on resize
                ui.move_window(view_switcher_id, view_switcher_pos);

                view_switcher(3, state_machine, ui, vec2(screen_width()-5., 35.));
            }
        );
    }

    
    fn state_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        
    }
}
