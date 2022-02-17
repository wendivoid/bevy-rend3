use bevy_app::prelude::*;
use bevy_math::prelude::*;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_transform::prelude::*;
use bevy_rend3::{Rend3Camera, Rend3, Rend3Skybox};

mod common;

fn main() {
    App::new()
        .add_plugins(common::ExamplePlugins)
        .add_startup_system(spawn_environment)
        .add_startup_system(spawn_cube)
        .run()
}

fn spawn_environment(
    rend3: Rend3,
    mut skybox: Rend3Skybox,
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

    skybox.set_texture(load_skybox_texture(&*rend3.renderer.0));
}

fn load_skybox_image(data: &mut Vec<u8>, path: &'static [u8]) {
    let decoded = image::load_from_memory(path)
        .unwrap()
        .into_rgba8();

    data.extend_from_slice(decoded.as_raw());
}

fn load_skybox_texture(
    renderer: &rend3::Renderer,
) -> rend3::types::TextureHandle {
    let mut data = Vec::new();
    load_skybox_image(&mut data, include_bytes!("resources/skybox/right.jpg"));
    load_skybox_image(&mut data, include_bytes!("resources/skybox/left.jpg"));
    load_skybox_image(&mut data, include_bytes!("resources/skybox/top.jpg"));
    load_skybox_image(&mut data, include_bytes!("resources/skybox/bottom.jpg"));
    load_skybox_image(&mut data, include_bytes!("resources/skybox/front.jpg"));
    load_skybox_image(&mut data, include_bytes!("resources/skybox/back.jpg"));

    renderer.add_texture_cube(rend3::types::Texture {
        format: rend3::types::TextureFormat::Rgba8UnormSrgb,
        size: UVec2::new(2048, 2048),
        data,
        label: Some("background".into()),
        mip_count: rend3::types::MipmapCount::ONE,
        mip_source: rend3::types::MipmapSource::Uploaded,
    })
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