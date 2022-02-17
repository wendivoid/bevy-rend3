use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_core::prelude::*;
use bevy_transform::prelude::*;

#[derive(Component)]
pub struct Rotates;

pub fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (std::f32::consts::PI / 5.0) * time.delta_seconds(),
        )) * *transform;
    }
}