pub extern crate rend3;
pub extern crate rend3_routine as routine;

mod plugin;
mod surface;
mod systems;
mod handle;
mod api;
mod ambient;
mod skyboxes;
mod events;

pub use self::handle::Rend3Handle;
pub use self::plugin::Rend3Plugin;
pub use self::ambient::AmbientLight;
pub use self::events::Rend3SurfaceCreated;
pub use self::api::{Rend3, Rend3Camera, Rend3Skybox};
pub use self::surface::{Rend3Surface, Rend3Surfaces};
pub use self::skyboxes::{Skybox, SkyBoxes};

use std::sync::Arc;
use pollster::block_on;
use rend3::types::Handedness;
use rend3::{create_iad, RendererInitializationError};

pub struct Instance(pub(crate) Arc<wgpu::Instance>);
pub struct Adapter(pub(crate) Arc<wgpu::Adapter>);
pub struct Device(pub(crate) Arc<wgpu::Device>);
pub struct Renderer(pub Arc<rend3::Renderer>);

pub fn initialize() -> Result<(Instance, Adapter, Device, Renderer), RendererInitializationError> {
    let iad = block_on(create_iad(None, None, None, None))?;

    let instance = iad.instance.clone();
    let device = iad.device.clone();
    let adapter = iad.adapter.clone();

    let renderer = rend3::Renderer::new(iad, Handedness::Left, None)?;

    Ok((Instance(instance), Adapter(adapter), Device(device), Renderer(renderer)))
}