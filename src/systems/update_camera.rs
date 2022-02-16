use bevy_ecs::prelude::*;
use rend3::types::Camera;

use crate::Renderer;

pub fn update_camera(
    camera: Res<Camera>,
    renderer: Res<Renderer>
) {
    renderer.0.set_camera_data(*camera);
}