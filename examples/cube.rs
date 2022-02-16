use bevy::prelude::*;
use rend3_routine::pbr::{PbrMaterial, AlbedoComponent};
use bevy_rend3::{Rend3Camera, Rend3, Rend3Plugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Rend3Plugin)
        .add_startup_system(spawn_environment)
        .add_startup_system(spawn_cube)
        .add_system(rotator_system)
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
        Rotates,
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

#[derive(Component)]
struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (std::f32::consts::PI / 5.0) * time.delta_seconds(),
        )) * *transform;
    }
}