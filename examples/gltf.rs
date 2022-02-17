use bevy::app::prelude::*;
use bevy::ecs::prelude::*;
use bevy::core::prelude::*;
use bevy::math::prelude::*;
use bevy::transform::prelude::*;
use rend3::types::Mesh;
use rend3_routine::pbr::PbrMaterial;
use bevy_rend3::{Rend3Camera, Rend3, Rend3Plugin};

mod common;

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
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
        intensity: 4.0,
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
    let (mesh, material) = load_gltf("examples/resources/rend3.glb");
    commands.spawn_bundle((
        common::Rotates,
        Transform::identity(),
        GlobalTransform::identity(),
        rend3.add_mesh(mesh),
        rend3.add_material(material)
    ));
}


fn load_gltf(
    path: &'static str,
) -> (Mesh, PbrMaterial) {
    let (doc, datas, _) = gltf::import(path).unwrap();
    let mesh_data = doc.meshes().next().expect("no meshes in data.glb");

    let primitive = mesh_data.primitives().next().expect("no primitives in data.glb");
    let reader = primitive.reader(|b| Some(&datas.get(b.index())?.0[..b.length()]));

    let vertex_positions: Vec<_> = reader.read_positions().unwrap().map(Vec3::from).collect();
    let vertex_normals: Vec<_> = reader.read_normals().unwrap().map(Vec3::from).collect();
    let vertex_tangents: Vec<_> = reader
        .read_tangents()
        .unwrap()
        .map(Vec4::from)
        .map(Vec4::truncate)
        .collect();
    let vertex_uvs: Vec<_> = reader
        .read_tex_coords(0)
        .unwrap()
        .into_f32()
        .map(Vec2::from)
        .collect();
    let indices = reader.read_indices().unwrap().into_u32().collect();

    let mesh = rend3::types::MeshBuilder::new(vertex_positions.to_vec(), rend3::types::Handedness::Right)
        .with_vertex_normals(vertex_normals)
        .with_vertex_tangents(vertex_tangents)
        .with_vertex_uv0(vertex_uvs)
        .with_indices(indices)
        .build()
        .unwrap();

    // Add basic material with all defaults except a single color.
    let material = primitive.material();
    let metallic_roughness = material.pbr_metallic_roughness();
    let material = rend3_routine::pbr::PbrMaterial {
        albedo: rend3_routine::pbr::AlbedoComponent::Value(metallic_roughness.base_color_factor().into()),
        ..Default::default()
    };

    (mesh, material)
}