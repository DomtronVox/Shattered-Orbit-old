use macroquad::{
    math::{vec3},
    color::Color,
    models::{draw_line_3d},
};

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
