use specs::{Component, VecStorage};


///Component representing physical tranformation (translation and rotation) of a default object.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TransformComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}
