use bevy::prelude::*;
use rend3_routine::pbr::{PbrMaterial, AlbedoComponent};
use bevy_rend3::{Rend3Camera, Rend3, Rend3Plugin};

mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Rend3Plugin)
        .add_plugin(common::ExamplePlugin)
        .add_startup_system(spawn_environment)
        .add_startup_system(spawn_cube)
        .run()
}

fn spawn_environment(
    rend3: Rend3,
    mut camera: Rend3Camera,
    mut commands: Commands,
) {
    let light = rend3::types::DirectionalLight {
        color: Vec3::ONE,
        intensity: 10.0,
        // Direction will be normalized
        direction: Vec3::new(-1.0, -4.0, 2.0),
        distance: 400.0,
    };

    commands.spawn_bundle((
        rend3.add_directional_light(light),
        Name::new("Directional Light")
    ));

    let view_location = Vec3::new(3.0, 3.0, -5.0);
    let view = Mat4::from_euler(EulerRot::XYZ, -0.55, 0.5, 0.0);
    camera.set_matrix(view * Mat4::from_translation(-view_location));


    camera.set_projection(rend3::types::CameraProjection::Perspective { vfov: 60.0, near: 0.1 });
}

fn spawn_cube(
    rend3: Rend3,
    mut commands: Commands
) {
    commands.spawn_bundle((
        common::Rotates,
        Transform::identity(),
        GlobalTransform::identity(),
        rend3.add_mesh(common::cube_mesh()),
        rend3.add_material(common::teal_material())
    ));
}