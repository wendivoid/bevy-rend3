use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_utils::tracing::error;
use rend3::types::Camera;
use rend3_routine::base::BaseRenderGraph;

use crate::{Rend3Surfaces, AmbientLight, SkyBoxes};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, SystemLabel)]
pub enum RenderSystem {
    UpdateCamera,
    Render
}

pub struct Rend3Plugin;

impl Plugin for Rend3Plugin {
    fn build(&self, app: &mut App) {
        match crate::initialize() {
            Err(err) => error!("Failed to initialize rend3: {:?}", err),
            Ok((instance, adapter, device, renderer)) => {
                let base_rendergraph = BaseRenderGraph::new(&renderer.0);

                let mut data_core = renderer.0.data_core.lock();
                let pbr_routine = rend3_routine::pbr::PbrRoutine::new(&renderer.0, &mut data_core, &base_rendergraph.interfaces);
                drop(data_core);

                app.insert_resource(instance)
                    .insert_resource(adapter)
                    .insert_resource(device)
                    .insert_resource(renderer)
                    .insert_resource(base_rendergraph)
                    .insert_resource(pbr_routine);
            }
        }
        app.init_resource::<Rend3Surfaces>()
            .init_resource::<SkyBoxes>()
            .init_resource::<Camera>()
            .init_resource::<AmbientLight>()
            .add_system(crate::systems::create_surface)
            .add_system(crate::systems::resize_surface)
            .add_system(crate::systems::create_object)
            .add_system(crate::systems::transform_object)
            .add_system_to_stage(
                CoreStage::Last,
                crate::systems::update_camera.label(RenderSystem::UpdateCamera)
            )
            .add_system_to_stage(
                CoreStage::Last,
                crate::systems::render_surface.label(RenderSystem::Render)
                    .after(RenderSystem::UpdateCamera)
            );
    }
}
