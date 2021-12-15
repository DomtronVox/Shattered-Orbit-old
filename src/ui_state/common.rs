use super::{StateEvent, UIStateMachine, 
    StationViewState, OrbitalViewState, PlanetViewState, MissionViewState};

use macroquad::{
    math::{vec2, vec3, Vec2, Vec3, Quat},
    ui::{hash, Ui, widgets},
    input::{is_key_down, KeyCode},
    models::{draw_line_3d},
    camera::Camera3D,
    color::Color,
};






//UI stuff for the common view_switcher panel
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


/// Draw a grid centered at (0, 0, 0) code from macroquad to fix grid orientation
pub fn draw_grid(slices: u32, spacing: f32, axes_color: Color, other_color: Color) {
    let half_slices = (slices as i32) / 2;
    for i in -half_slices..half_slices + 1 {
        let color = if i == 0 { axes_color } else { other_color };

        draw_line_3d(
            vec3(i as f32 * spacing, -half_slices as f32 * spacing, 0.),
            vec3(i as f32 * spacing, half_slices as f32 * spacing, 0.),
            color,
        );
        draw_line_3d(
            vec3(-half_slices as f32 * spacing, i as f32 * spacing, 0.),
            vec3(half_slices as f32 * spacing, i as f32 * spacing, 0.),
            color,
        );
    }
}



pub fn handle_camera_controls(camera: &mut Camera3D) {
    //handle camera movement controls
    let mut dir = (0., 0.);
    let mut rot_speed = 0.01;
    let mut zoom_speed = 0.;
    
    //figure out direction of movement
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        dir.0 -= 1.;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        dir.0 += 1.;
    }
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        dir.1 -= 1.;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        dir.1 += 1.;
    }
    
    let is_shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
    
    //figure out direction of zoom
    if is_key_down(KeyCode::Q) {
        if ! is_shift {
            zoom_speed += 0.99;
        } else {
            zoom_speed += 0.90;
        }            
    }
    if is_key_down(KeyCode::E) {
        if ! is_shift {
            zoom_speed += 1.01;
        } else {
            zoom_speed += 1.10;
        }
    }
    
    //toggle fast rotation
    if is_shift {
        rot_speed += 0.10;
    }
    
    
    //handle calculating new camera position when player trys to move camera.
    if dir != (0., 0.) {
        //We need to calculate rotating the camera around the target
        
        //Adjust camera position as if it was rotating around origin.
        let adjusted_pos = camera.position - camera.target;
        
        //Left-Right rotates us around the z axis
        let z_rotation = Quat::from_rotation_z( dir.0 * rot_speed );
        
        //Up-Down rotates us around an axis perpendicular to current view and Z axis
        let axis = Vec3::Z.cross( adjusted_pos.normalize() ).normalize();
        let xy_rotation = Quat::from_axis_angle( axis, (dir.1 * rot_speed) );
        
        //TODO need to limit xy rotation

        //merge the two rotations
        let rotation = z_rotation.mul_quat(xy_rotation);
        
        //rotate our adjusted position then readjust back to the target relative origin
        camera.position = rotation.mul_vec3(adjusted_pos) + camera.target;
    }
    
    //handle zoom if player trys to zoom
    if zoom_speed != 0. {
        //zoom in
        if zoom_speed < 1. && camera.position.length() > 2. {
            camera.position = camera.position * zoom_speed;
        }
        
        //zoom out
        if zoom_speed > 1. && camera.position.length() < 30. {
            camera.position = camera.position * zoom_speed;
        }
    }
}
