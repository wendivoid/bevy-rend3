use bevy_ecs::prelude::*;
use bevy_math::UVec2;
use bevy_utils::tracing::debug;
use bevy_window::{WindowId, WindowResized};

use crate::{Rend3Surfaces, Renderer, Device};

pub fn resize_surface(
    mut cache: Local<Vec<(WindowId, f32, f32)>>,
    renderer: Res<Renderer>,
    device: Res<Device>,
    surfaces: Res<Rend3Surfaces>,
    mut events: EventReader<WindowResized>
) {
     let mut to_remove = vec![];
     for (dex, (id, width, height)) in cache.iter().enumerate() {
         if let Some(surface) = surfaces.surfaces.get(id) {
             rend3::configure_surface(
                 &surface.surface,
                 &device.0,
                 surface.format,
                 UVec2::new(*width as u32, *height as u32),
                 rend3::types::PresentMode::Mailbox,
             );

             renderer.0.set_aspect_ratio(width / height);
             to_remove.push(dex);
         }
     }
     for remove in to_remove.into_iter() {
         cache.remove(remove);
     }
     for WindowResized { id, width, height } in events.iter() {
         debug!("Resizing surface on window: {id:?}, ({width}, {height})");
         if let Some(surface) = surfaces.surfaces.get(&id) {
             rend3::configure_surface(
                 &surface.surface,
                 &device.0,
                 surface.format,
                 UVec2::new(*width as u32, *height as u32),
                 rend3::types::PresentMode::Mailbox,
             );

             renderer.0.set_aspect_ratio(width / height);
         } else {
             cache.push((*id, *width, *height));
         }
     }
}