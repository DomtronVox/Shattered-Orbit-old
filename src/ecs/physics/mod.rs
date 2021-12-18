mod transform_component;
pub use transform_component::TransformComponent;

mod orbit;
pub use orbit::{Orbit, OrbitMotionSystem};

use specs::{Dispatcher, DispatcherBuilder};


///Builds a dispatcher that will update all graphics related systems.
pub fn build_physics_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    .with(OrbitMotionSystem{}, "OrbitMotionSystem", &[])
    .build()
}

