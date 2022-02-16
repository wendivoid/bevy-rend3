use bevy::prelude::*;
use rend3_routine::pbr::{PbrMaterial, AlbedoComponent};
use bevy_rend3::{Rend3Camera, Rend3, Rend3Skybox, Rend3Plugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Rend3Plugin)
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
        Transform::identity(),
        GlobalTransform::identity(),
        rend3.add_mesh(mesh()),
        rend3.add_material(material())
    ));
}

fn material() -> PbrMaterial {
    PbrMaterial {
        albedo: AlbedoComponent::Value(Vec4::new(0.0, 0.5, 0.5, 1.0)),
        ..PbrMaterial::default()
    }
}

fn vertex(pos: [f32; 3]) -> Vec3 {
    Vec3::from(pos)
}

fn mesh() -> rend3::types::Mesh {
    let vertex_positions = [
        // far side (0.0, 0.0, 1.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        // near side (0.0, 0.0, -1.0)
        vertex([-1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // right side (1.0, 0.0, 0.0)
        vertex([1.0, -1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        // left side (-1.0, 0.0, 0.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // top (0.0, 1.0, 0.0)
        vertex([1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        // bottom (0.0, -1.0, 0.0)
        vertex([1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
    ];

    let index_data: &[u32] = &[
        0, 1, 2, 2, 3, 0, // far
        4, 5, 6, 6, 7, 4, // near
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // top
        20, 21, 22, 22, 23, 20, // bottom
    ];

    rend3::types::MeshBuilder::new(vertex_positions.to_vec(), rend3::types::Handedness::Left)
        .with_indices(index_data.to_vec())
        .build()
        .unwrap()
}