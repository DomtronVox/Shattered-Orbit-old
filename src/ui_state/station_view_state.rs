use super::{UIState, StateEvent, UIStateMachine, view_switcher};
use crate::Simulation;



use macroquad::{
    math::vec2,
    window::{screen_width, screen_height},
    ui::{hash, root_ui, widgets},
};




pub struct StationViewState {}

impl StationViewState {
    pub fn new() -> StationViewState {
        StationViewState {}
    }
}

impl UIState for StationViewState {

    fn world_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        
    }
    

    fn ui_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        let side_menu_id = hash!();
        
        //calculate size and position of side panel
        let side_size = vec2(screen_width() / 3., screen_height() - 38.);
        let side_pos = vec2(screen_width() - (side_size.x) , 0. );
        
        //Setup a window that will hold side menu options.
        root_ui().window(side_menu_id, side_pos, side_size,         
            |ui| {
                //make sure to update window position so it says in the right place on resize
                ui.move_window(side_menu_id, side_pos);
                
                
            },
        );

        //setup View Switcher and containing window
        let view_switcher_id = hash!();
        let view_switcher_pos = vec2(0., screen_height() - 40.);
        
        root_ui().window(view_switcher_id, view_switcher_pos, vec2(screen_width(), 40.),
            |ui| { 
                //make sure to update window position so it says in the right place on resize
                ui.move_window(view_switcher_id, view_switcher_pos);

                view_switcher(0, state_machine, ui, vec2(screen_width()-5., 35.));
            }
        );
    }
    
}
