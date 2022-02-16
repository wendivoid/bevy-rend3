use bevy_utils::HashMap;
use bevy_window::WindowId;
use rend3::types::Texture;
use routine::skybox::SkyboxRoutine;

use crate::Rend3Handle;

pub struct Skybox {
    pub routine: SkyboxRoutine,
    pub texture: Option<Rend3Handle<Texture>>
}

#[derive(Default)]
pub struct SkyBoxes {
    pub sky_boxes: HashMap<WindowId, Skybox>
}