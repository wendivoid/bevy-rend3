use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_math::Mat4;
use bevy_window::WindowId;
use rend3::types::{Camera, CameraProjection, DirectionalLight, Material, MaterialTag, Mesh, Object, ResourceHandle, Texture};
use routine::base::BaseRenderGraph;
use routine::skybox::SkyboxRoutine;

use crate::{Renderer, Instance, Device, Adapter, Rend3Handle, SkyBoxes, Skybox};

#[derive(SystemParam)]
pub struct Rend3<'w, 's> {
    pub renderer: Res<'w, Renderer>,
    pub(crate) instance: Res<'w, Instance>,
    pub(crate) device: Res<'w, Device>,
    pub(crate) adapter: Res<'w, Adapter>,
    #[system_param(ignore)]
    _phantom: std::marker::PhantomData<&'s ()>
}

impl <'w, 's>Rend3<'w, 's> {

    pub fn add_mesh(&self, mesh: Mesh) -> Rend3Handle<Mesh> {
        self.renderer.0.add_mesh(mesh).into()
    }

    pub fn add_material<M: Material>(&self, material: M) -> Rend3Handle<MaterialTag> {
        self.renderer.0.add_material(material).into()
    }

    pub fn add_directional_light(&self, light: DirectionalLight) -> Rend3Handle<DirectionalLight> {
        self.renderer.0.add_directional_light(light).into()
    }

    pub fn add_object(&self, object: Object) -> Rend3Handle<Object> {
        self.renderer.0.add_object(object).into()
    }

    pub fn set_object_transform(&self, handle: &Rend3Handle<Object>, object: Mat4) {
        self.renderer.0.set_object_transform(&handle.0, object);
    }
}

#[derive(SystemParam)]
pub struct Rend3Skybox<'w, 's> {
    renderer: Res<'w, Renderer>,
    skyboxes: ResMut<'w, SkyBoxes>,
    base_render_graph: Res<'w, BaseRenderGraph>,
    #[system_param(ignore)]
    _phantom: std::marker::PhantomData<&'s ()>
}

impl <'w, 's> Rend3Skybox<'w, 's> {
    pub fn set_texture(&mut self, handle: ResourceHandle<Texture>) {
        self.set_surface_texture(WindowId::primary(), handle);
    }
    pub fn set_surface_texture(&mut self, window_id: WindowId, handle: ResourceHandle<Texture>) {
        if let Some(skybox) = self.skyboxes.sky_boxes.get_mut(&window_id) {
            skybox.routine.set_background_texture(Some(handle));
        } else {
            let mut skybox = SkyboxRoutine::new(&self.renderer.0, &self.base_render_graph.interfaces);
            skybox.set_background_texture(Some(handle.clone()));
            self.skyboxes.sky_boxes.insert(window_id, Skybox {
                routine: skybox,
                texture: Some(Rend3Handle(handle))
            });
        }
    }
}

#[derive(SystemParam)]
pub struct Rend3Camera<'w, 's> {
    camera: ResMut<'w, Camera>,
    #[system_param(ignore)]
    _phantom: std::marker::PhantomData<&'s ()>
}

impl <'w, 's> Rend3Camera<'w, 's> {
    pub fn set_matrix(&mut self, matrix: Mat4) {
        self.camera.view = matrix;
    }

    pub fn set_projection(&mut self, projection: CameraProjection) {
        self.camera.projection = projection;
    }
}