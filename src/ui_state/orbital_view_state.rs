use super::{UIState, StateEvent, UIStateMachine, view_switcher};
use crate::Simulation;


use std::f32::consts::PI;
use macroquad::{
    math::{vec2, vec3, Vec3, Quat},
    window::{screen_width, screen_height},
    ui::{hash, root_ui, widgets},
    input::{is_key_down, KeyCode},
    camera::{Camera3D, set_camera},
    models::draw_grid,
    color::colors::WHITE,
};




pub struct OrbitalViewState {
    camera: Camera3D,
}

impl OrbitalViewState {
    pub fn new() -> OrbitalViewState {
        OrbitalViewState {
            camera: Camera3D{
                target: vec3(10., 0., 0.),
                position: vec3(5., 0., 0.),
                ..Default::default()
            },
        }
    }
}

impl UIState for OrbitalViewState {

    fn world_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
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
        
        //figure out direction of zoom
        if is_key_down(KeyCode::Q) {
            zoom_speed += 0.99;
        }
        if is_key_down(KeyCode::E) {
            zoom_speed -= 0.99;
        }
        
        //toggle fast rotation
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            rot_speed += 0.04;
            zoom_speed *= 2.;
        }
        
        
        //handle calculating new camera position when player trys to move camera.
        if dir != (0., 0.) {
            //We need to calculate rotating the camera around the target
            
            //Adjust camera position as if it was rotating around origin.
            let adjusted_pos = self.camera.position - self.camera.target;
            
            //Left-Right rotates us around the z axis
            let z_rotation = Quat::from_rotation_z( dir.0 * rot_speed );
            
            //Up-Down rotates us around an axis perpendicular to current view and Z axis
            let axis = Vec3::Z.cross( adjusted_pos.normalize() );
            let xy_rotation = Quat::from_axis_angle( axis, (dir.1 * rot_speed) );
            
            //TODO need to limmit xy roa

            //merge the two rotations
            let rotation = z_rotation.mul_quat(xy_rotation);
            
            //rotate our adjusted position then readjust back to the target relative origin
            self.camera.position = rotation.mul_vec3(adjusted_pos) + self.camera.target;
        }
        
        //handle zoom if player trys to zoom
        if zoom_speed > 0. {
            //can just multiply position by a scaler
            //self.camera.position *= zoom_speed;
        }
        
        //set active camera
        set_camera(&self.camera);
        
        draw_grid(20, 2., WHITE, WHITE);
        
        //render valid entities onto camera
        sim.ecs.render_orbits()
    }

    fn ui_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        
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

    

}
