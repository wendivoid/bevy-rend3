use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use bevy_window::Windows;
use image::GenericImageView;
use bevy_rend3::{Rend3, Rend3Camera};

mod common;

fn main() {
    App::new()
        .add_plugins(common::ExamplePlugins)
        .add_startup_system(setup_environment)
        .add_startup_system(spawn_quad)
        .run()
}

fn setup_environment(
    mut camera: Rend3Camera,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    let view_location = Vec3::new(0.0, 0.0, -1.0);
    let view = Mat4::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
    let view = view * Mat4::from_translation(-view_location);

    camera.set_matrix(view);
    camera.set_projection(rend3::types::CameraProjection::Orthographic {
        size: bevy_math::Vec3A::new(
            window.width() as f32,
            window.height() as f32,
            10.0,
        ),
    });
}

fn spawn_quad(
    rend3: Rend3,
    mut commands: Commands
) {
    let image_checker =
        image::load_from_memory(include_bytes!("resources/checker.png")).expect("Failed to load image from memory");
    let image_checker_rgba8 = image_checker.to_rgba8();
    let texture_checker = rend3::types::Texture {
        label: Option::None,
        data: image_checker_rgba8.to_vec(),
        format: rend3::types::TextureFormat::Rgba8UnormSrgb,
        size: UVec2::new(image_checker.dimensions().0, image_checker.dimensions().1),
        mip_count: rend3::types::MipmapCount::ONE,
        mip_source: rend3::types::MipmapSource::Uploaded,
    };
    let texture_checker_handle = rend3.renderer.0.add_texture_2d(texture_checker);

    // Add PBR material with all defaults except a single color.
    let material = rend3_routine::pbr::PbrMaterial {
        albedo: rend3_routine::pbr::AlbedoComponent::Texture(texture_checker_handle),
        unlit: true,
        sample_type: rend3_routine::pbr::SampleType::Nearest,
        ..rend3_routine::pbr::PbrMaterial::default()
    };
    commands.spawn_bundle((
        rend3.add_mesh(create_quad(300.0)),
        rend3.add_material(material),
        Transform::identity(),
        GlobalTransform::identity()
    ));
}

fn vertex(pos: [f32; 3]) -> Vec3 {
    Vec3::from(pos)
}

fn uv(pos: [f32; 2]) -> Vec2 {
    Vec2::from(pos)
}

fn create_quad(size: f32) -> rend3::types::Mesh {
    let vertex_positions = [
        vertex([-size * 0.5, size * 0.5, 0.0]),
        vertex([size * 0.5, size * 0.5, 0.0]),
        vertex([size * 0.5, -size * 0.5, 0.0]),
        vertex([-size * 0.5, -size * 0.5, 0.0]),
    ];
    let uv_positions = [uv([0.0, 0.0]), uv([1.0, 0.0]), uv([1.0, 1.0]), uv([0.0, 1.0])];
    let index_data: &[u32] = &[0, 1, 2, 2, 3, 0];

    rend3::types::MeshBuilder::new(vertex_positions.to_vec(), rend3::types::Handedness::Left)
        .with_vertex_uv0(uv_positions.to_vec())
        .with_indices(index_data.to_vec())
        .build()
        .unwrap()
}