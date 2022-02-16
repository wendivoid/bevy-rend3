#![allow(dead_code)]

use bevy_app::prelude::*;

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