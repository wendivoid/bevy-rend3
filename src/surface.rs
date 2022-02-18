use std::sync::Arc;
use bevy_utils::HashMap;
use bevy_window::WindowId;
use rend3::types::TextureFormat;
use rend3_routine::tonemapping::TonemappingRoutine;

pub struct Rend3Surface {
    pub surface: Arc<wgpu::Surface>,
    pub format: TextureFormat,
    pub tone_mapping: TonemappingRoutine
}

#[derive(Default)]
pub struct Rend3Surfaces {
    pub surfaces: HashMap<WindowId, Rend3Surface>
}