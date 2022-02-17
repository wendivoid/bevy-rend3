use bevy_app::prelude::*;
use bevy_core::Time;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use rend3::types::CameraProjection;
use bevy_rend3::{Rend3, Rend3Camera};

mod common;

#[derive(Component)]
pub struct Armature(rend3_gltf::Armature);
#[derive(Component)]
pub struct SceneInstance(rend3_gltf::GltfSceneInstance);
#[derive(Component)]
pub struct Scene(rend3_gltf::LoadedGltfScene);

/// Locates an object in the node list that corresponds to an animated mesh
/// and returns its list of skeletons. Note that a gltf object may contain
/// multiple primitives, and there will be one skeleton per primitive.
pub fn find_armature(instance: &rend3_gltf::GltfSceneInstance) -> Option<rend3_gltf::Armature> {
    for node in &instance.nodes {
        if let Some(ref obj) = node.inner.object {
            if let Some(ref armature) = obj.inner.armature {
                return Some(armature.clone());
            }
        }
    }
    None
}

fn main() {
    App::new()
        .add_plugins(common::ExamplePlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_model)
        .add_system(update_armature)
        .run()
}

fn setup_camera(
    rend3: Rend3,
    mut commands: Commands,
    mut camera: Rend3Camera
) {
    let view_location = Vec3::new(0.0, 0.0, -10.0);
    let view = Mat4::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
    let view = view * Mat4::from_translation(-view_location);

    camera.set_matrix(view);
    camera.set_projection(CameraProjection::Perspective {
        vfov: 60.0, near: 0.1
    });

    let light = rend3::types::DirectionalLight {
        color: Vec3::ONE,
        intensity: 10.0,
        // Direction will be normalized
        direction: Vec3::new(-1.0, -4.0, 2.0),
        distance: 400.0,
    };
    commands.spawn().insert(rend3.add_directional_light(light));
}

fn setup_model(
    rend3: Rend3,
    mut commands: Commands
) {
    let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/resources/RiggedSimple.glb"));
    let gltf_data = std::fs::read(&path).unwrap();
    let parent_directory = path.parent().unwrap();
    let (loaded_scene, loaded_instance) = pollster::block_on(rend3_gltf::load_gltf(
        &rend3.renderer.0,
        &gltf_data,
        &rend3_gltf::GltfLoadSettings::default(),
        |p| rend3_gltf::filesystem_io_func(&parent_directory, p),
    )).expect("Loading gltf scene");

    commands.spawn_bundle((
        Armature(find_armature(&loaded_instance).unwrap()),
        SceneInstance(loaded_instance),
        Scene(loaded_scene)
    ));
}

fn update_armature(
    rend3: Rend3,
    time: Res<Time>,
    query: Query<(&Armature, &Scene)>
) {
    for (armature, scene) in query.iter() {
        let inverse_bind_matrices = &scene.0.skins[armature.0.skin_index].inner.inverse_bind_matrices;

        // Compute a very simple animation for the top bone
        let t = time.seconds_since_startup() as f32;
        let rotation_degrees = 30.0 * f32::sin(5.0 * t);

        // An armature contains multiple skeletons, one per mesh primitive being
        // deformed. We need to set the joint matrices per each skeleton.
        for skeleton in &armature.0.skeletons {
            rend3.renderer.0.set_skeleton_joint_transforms(
                skeleton,
                &[
                    Mat4::from_translation(Vec3::new(0.0, 0.0, -4.18)),
                    Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0))
                        * Mat4::from_rotation_x(rotation_degrees.to_radians()),
                ],
                inverse_bind_matrices,
            );
        }
    }
}