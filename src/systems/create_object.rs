use bevy_ecs::prelude::*;
use bevy_transform::prelude::GlobalTransform;
use bevy_utils::tracing::debug;
use rend3::types::{MaterialTag, Mesh, Object, ObjectMeshKind};

use crate::{Rend3, Rend3Handle};

pub fn create_object(
    rend3: Rend3,
    mut commands: Commands,
    query: Query<(Entity, &Rend3Handle<Mesh>, &Rend3Handle<MaterialTag>, &GlobalTransform), Without<Rend3Handle<Object>>>
) {
    for (entity, mesh, material, transform) in query.iter() {
        debug!("Creating Object for Entity: {entity:?}");
        commands.entity(entity).insert(rend3.add_object(Object {
            mesh_kind: ObjectMeshKind::Static(mesh.0.clone()),
            material: material.0.clone(),
            transform: transform.compute_matrix()
        }));
    }
}