use std::sync::Arc;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_utils::tracing::debug;
use bevy_window::WindowCreated;
use bevy_winit::WinitWindows;
use rend3_routine::base::BaseRenderGraph;
use rend3_routine::skybox::SkyboxRoutine;
use rend3_routine::tonemapping::TonemappingRoutine;

use crate::{Rend3, Surface, Surfaces};

pub fn create_surface(
    rend3: Rend3,
    windows: Res<WinitWindows>,
    base_render_graph: Res<BaseRenderGraph>,
    mut surfaces: ResMut<Surfaces>,
    mut events: EventReader<WindowCreated>
) {
    for WindowCreated { id } in events.iter() {
        let window = windows.get_window(*id).unwrap();
        debug!("Creating Surface for window: {id:?}");

        let window_size = window.inner_size();

        let surface = Arc::new(unsafe { rend3.instance.0.create_surface(&window) });

        let format = surface.get_preferred_format(&rend3.adapter.0).unwrap();
        // Configure the surface to be ready for rendering.
        rend3::configure_surface(
            &surface,
            &rend3.device.0,
            format,
            UVec2::new(window_size.width, window_size.height),
            rend3::types::PresentMode::Mailbox,
        );

        let tone_mapping =
            TonemappingRoutine::new(&rend3.renderer.0, &base_render_graph.interfaces, format);

        let skybox = SkyboxRoutine::new(&rend3.renderer.0, &base_render_graph.interfaces);

        surfaces.surfaces.insert(*id, Surface {
            surface, format, tone_mapping,
            skybox
        });
    }
}