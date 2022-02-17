#![allow(dead_code)]

use bevy_app::prelude::*;
use bevy_app::PluginGroupBuilder;

mod cube;
mod teal;
mod rotates;

pub use self::rotates::Rotates;
pub use self::cube::cube_mesh;
pub use self::teal::teal_material;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rotates::rotator_system);
    }
}

pub struct ExamplePlugins;

impl PluginGroup for ExamplePlugins {
    fn build(&mut self, app: &mut PluginGroupBuilder) {
        app.add(bevy_rend3::Rend3Plugin);
        app.add(bevy_core::CorePlugin);
        app.add(bevy_input::InputPlugin::default());
        app.add(bevy_app::ScheduleRunnerPlugin::default());
        app.add(bevy_window::WindowPlugin::default());
        app.add(bevy_winit::WinitPlugin::default());
        app.add(bevy_transform::TransformPlugin::default());
        app.add(ExamplePlugin);
    }
}