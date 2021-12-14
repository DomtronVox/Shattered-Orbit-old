use super::{StateEvent, UIStateMachine, 
    StationViewState, OrbitalViewState, PlanetViewState, MissionViewState};

use macroquad::{
    math::Vec2,
    ui::{hash, Ui, widgets},
};







pub fn view_switcher(current_tab: u32, state_machine: &mut UIStateMachine, 
        ui: &mut Ui, size: Vec2) {

    let tabs = ["Station Status", "Orbital Tracking", 
                "Planet Survay", "Mission Planning"];

    //create a new event based on what view switch was pressed
    state_machine.handle_event(
        match widgets::Tabbar::new(hash!(), size, &tabs)
            //TODO silly +0 to copy int, there is probebly a better way.
            .selected_tab(Some(&mut (current_tab+0))) 
            .ui(ui)
        {   
            x if x == 0 && x != current_tab =>
                 StateEvent::ChangeState( Box::new(StationViewState::new()) ),
            x if x == 1 && x != current_tab =>
                 StateEvent::ChangeState( Box::new(OrbitalViewState::new()) ),
            x if x == 2 && x != current_tab =>
                 StateEvent::ChangeState( Box::new(PlanetViewState::new()) ),
            x if x == 3 && x != current_tab =>
                 StateEvent::ChangeState( Box::new(MissionViewState::new()) ),
            _ => StateEvent::None
            
        }
    );
}
