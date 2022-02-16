use bevy_math::prelude::*;

pub struct AmbientLight(pub Vec4);

impl Default for AmbientLight {
    fn default() -> AmbientLight {
        AmbientLight(Vec4::ZERO)
    }
}