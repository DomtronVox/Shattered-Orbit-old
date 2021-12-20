use super::{UIState, StateEvent, UIStateMachine, view_switcher, handle_camera_controls};
use crate::Simulation;
use crate::extend_microquad_gfx::draw_grid;

use std::f32::consts::PI;
use macroquad::{
    math::{vec2, vec3, Vec3, Quat},
    window::{screen_width, screen_height},
    ui::{hash, root_ui, widgets},
    models::{draw_line_3d},
    input::{is_key_down, KeyCode},
    camera::{Camera3D, set_camera},
    color::colors::{WHITE, YELLOW},
};




pub struct OrbitalViewState {
    camera: Camera3D,
}

impl OrbitalViewState {
    pub fn new() -> OrbitalViewState {
        OrbitalViewState {
            camera: Camera3D{
                target: vec3(0., 0., 0.),
                position: vec3(0., -0.1, 20.),
                ..Default::default()
            },
        }
    }
}

impl UIState for OrbitalViewState {

    fn world_render(&mut self, state_machine: &mut UIStateMachine, sim: &mut Simulation) {
        
        //check for camera control input and update camera accordingly
        handle_camera_controls(&mut self.camera);
        
        //set active camera
        set_camera(&self.camera);
        
        //draw_grid(20, 2., WHITE, WHITE);
        draw_line_3d(Vec3::ZERO, Vec3::X*100., YELLOW); //Line pointing in +X Axis
        
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
