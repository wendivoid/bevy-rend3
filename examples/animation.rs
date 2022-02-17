use bevy_app::prelude::*;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use rend3::types::CameraProjection;
use bevy_rend3::{Rend3, Rend3Camera};

mod common;

#[derive(Component)]
pub struct AnimationTime(f32);
#[derive(Component)]
pub struct Animation(rend3_anim::AnimationData);
#[derive(Component)]
pub struct SceneInstance(rend3_gltf::GltfSceneInstance);
#[derive(Component)]
pub struct Scene(rend3_gltf::LoadedGltfScene);


fn main() {
    App::new()
        .add_plugins(common::ExamplePlugins)
        .add_startup_system(setup_environment)
        .add_startup_system(setup_model)
        .add_system(animate_model)
        .run()
}

fn setup_environment(
    rend3: Rend3,
    mut commands: Commands,
    mut camera: Rend3Camera,
) {
    let view_location = Vec3::new(0.0, 1.5, -5.0);
    let view = Mat4::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
    let view = view * Mat4::from_translation(-view_location);

    camera.set_matrix(view);
    camera.set_projection(CameraProjection::Perspective { vfov: 60.0, near: 0.1 });

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
    let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/resources/animated/scene.gltf"));
    let gltf_data = std::fs::read(&path).unwrap();
    let parent_directory = path.parent().unwrap();
    let (loaded_scene, loaded_instance) = pollster::block_on(rend3_gltf::load_gltf(
        &rend3.renderer.0,
        &gltf_data,
        &rend3_gltf::GltfLoadSettings::default(),
        |p| rend3_gltf::filesystem_io_func(&parent_directory, p),
    )).expect("Loading gltf scene");

    commands.spawn_bundle((
        Animation(rend3_anim::AnimationData::from_gltf_scene(&loaded_scene, &loaded_instance)),
        Scene(loaded_scene),
        SceneInstance(loaded_instance),
        AnimationTime(0.0)
    ));
}

fn animate_model(
    rend3: Rend3,
    time: Res<Time>,
    mut query: Query<(&Animation, &mut AnimationTime, &Scene, &SceneInstance)>
) {
    for (animation, mut anim_time, scene, instance) in query.iter_mut() {
        anim_time.0 = (anim_time.0 + time.delta_seconds()) % scene.0.animations[0].inner.duration;
        rend3_anim::pose_animation_frame(
            &rend3.renderer.0,
            &scene.0,
            &instance.0,
            &animation.0,
            0,
            anim_time.0,
        )
    }
}