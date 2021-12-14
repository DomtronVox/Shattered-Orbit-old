mod satellite_gfx;
pub use satellite_gfx::{SatelliteGFX, OrbitDisplaySystem};

use specs::{Dispatcher, DispatcherBuilder};


///Builds a dispatcher that will update all graphics related systems.
pub fn build_render_orbits_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    .with(OrbitDisplaySystem{}, "OrbitDisplaySystem", &[])
    .build()
}


