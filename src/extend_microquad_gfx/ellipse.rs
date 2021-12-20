use macroquad::{
    math::{vec3, Vec3, Quat},
    color::Color,
    models::{draw_line_3d},
};

use std::f64::consts::PI;

/// calculate an x/y point of an ellipse at a given angle
fn caclulate_vertex_at_angle(a: f64, b: f64, angle: f64) -> Vec3 {
    //if (angle

    let mut x = (a*b) / ( b.powi(2) + a.powi(2) * angle.tan().powi(2) ).sqrt();
    let mut y = (a*b) / ( a.powi(2) + b.powi(2) / angle.tan().powi(2) ).sqrt();

    //adjust x's sign based on where angle lies
    if angle > PI/2. && angle < (3.*PI)/2. { x *= -1.; }
    if angle > PI { y *= -1.; }
    
    vec3(x as f32, y as f32, 0.)
}

///Draw an orbital ellipse via line segments
pub fn draw_ellipse(semi_major: f64, eccentricity: f64, inclination: f64, longitude: f64, 
        color: Color) {
    //we need to calculate some more info from the given info to work with the ellipse
    let foci_mag = eccentricity * semi_major;
    let semi_minor = ( semi_major.powi(2) - foci_mag.powi(2) ).sqrt();

    //offset so drawing center is away from primaries foci
    let center_offset = vec3( -foci_mag as f32, 0., 0. );
    
    //we need to setup the adjustment that will make this ellipse 3D
    let rotation = Quat::from_rotation_y(inclination as f32).mul_quat(
                    Quat::from_rotation_z(longitude as f32)
                );
    
    //calculate the very first vertex that we will draw a line from
    let mut last_vertex = vec3( (semi_major) as f32, 0., 0. );
    last_vertex += center_offset; //offset drawing center
    //> we also need to rotate it to be in the correct 3D space
    last_vertex = rotation.mul_vec3(last_vertex);
    
    //calculate angle step from number of segments. each step will be a line
    let angle_step = (2.*PI) / 100.; //do 100 lines to approximate the ellipse

    //we don't start from 0 because we already calculated first vertex above
    let mut angle = angle_step; 
    
    while angle <= (2.*PI) {
        let mut vertex = caclulate_vertex_at_angle(semi_major, semi_minor, angle);
        vertex += center_offset; //offset drawing center
        vertex = rotation.mul_vec3(vertex);
        
        //draw the segment
        //TODO should probably buffer all segments at once then draw
        draw_line_3d(last_vertex, vertex, color);
        
        //update last vertex
        last_vertex = vertex;
        
        //advance angle by a step
        angle += angle_step;
    }
    
}
