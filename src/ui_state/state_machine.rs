use std::collections::HashMap;

use crate::Simulation;

use crate::ui_state::MainMenuState;

use macroquad::camera::set_default_camera;


///The game state trait lets us handle different stages of the application in a modular
///  way by simply changing to a new state as needed.
pub trait UIState {

    fn update(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        self.world_render(state_machine, sim);

        //set default camera before drawing UI stuff
        set_default_camera();
        self.ui_render(state_machine, sim);
    }

    //code that handles displaying the world
    fn world_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation);
    
    //code that handles user interface logic
    fn ui_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation);
}


///Dictates changes to the state machine
pub enum StateEvent {
    None,
    Shutdown,
    ChangeState(Box<dyn UIState>),
}


///Tracks all states and handles switching
pub struct UIStateMachine {
    states: HashMap<String, Box<dyn UIState>>,
    current_state: Option<Box<dyn UIState>>,
    pub running: bool,
}


impl UIStateMachine {

    //Construct a default State Machine
    pub fn new() -> Self {
        UIStateMachine {
            states: HashMap::new(),
            current_state: Some(Box::new(MainMenuState::new())),
            running: true,
        }
    }

    ///Run the current state's update function
    pub fn update(&mut self, sim: &mut Simulation) {
        let mut current_state = self.current_state.take().expect("Error invalid ui state.");

        current_state.update(self, sim);

        //only assign back to current_state if the state wasn't changed during the update.
        if self.current_state.is_none() {
            self.current_state = Some(current_state);
        }
    }

    ///Handles a StateEvent event.
    pub fn handle_event(&mut self, event: StateEvent) {
        match event {
            StateEvent::None => return,
            StateEvent::Shutdown => self.running = false,
            //todo update this to use the states hashmap
            StateEvent::ChangeState(new_state) => self.current_state = Some(new_state),
        }
    }
}
