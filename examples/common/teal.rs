use bevy_math::prelude::*;
use rend3_routine::pbr::{AlbedoComponent, PbrMaterial};

pub fn teal_material() -> PbrMaterial {
    PbrMaterial {
        albedo: AlbedoComponent::Value(Vec4::new(0.0, 0.5, 0.5, 1.0)),
        ..PbrMaterial::default()
    }
}