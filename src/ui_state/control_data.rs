


/// Controller input values used by different ECS Systems
pub struct ControlData {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
}

impl ControlData {
    pub fn new() -> Self {
        ControlData {
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
        }
    }
}
