use super::super::physics::TransformComponent;


use specs::{
    System, 
    Component, 
    VecStorage, 
    ReadStorage, 
    Join,
};

use macroquad::{
    models::draw_sphere,
    color::Color,
    math::vec3,
};


///Component that holds graphical information for an orbital body.
///Note that mass should be in Kg and Vectors are in meters.
#[derive(Component)]
#[storage(VecStorage)]
pub struct SatelliteGFX {
    //model: ModelType, //aka sphere for planets, random convex shape for asteroids/etc, or decal for ships/stations
    //texture: ?, //Texture to apply to the above model.
    pub color: Color, //plain color to apply to sphre in lieu of a texture.
}


///System that handles advancing the position of all Satellites
pub struct OrbitDisplaySystem;
impl<'a> System<'a> for OrbitDisplaySystem {

    type SystemData = (ReadStorage<'a, TransformComponent>,
                       ReadStorage<'a, SatelliteGFX>);
                       

    fn run(&mut self, (transforms, graphics): Self::SystemData) {

        for (transform, graphic) in (&transforms, &graphics).join() {
            //macroquad uses GLM instead of nalgebra_glm so we have to translate types
            let position = vec3( (transform.x*10.) as f32,
                                 (transform.y*10.) as f32,
                                 (transform.z*10.) as f32);

            draw_sphere(position, 1., None, graphic.color);

        }
    
    }
}
