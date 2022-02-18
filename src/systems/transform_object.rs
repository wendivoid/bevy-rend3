use bevy_ecs::prelude::*;
use rend3::types::Object;
use bevy_transform::prelude::GlobalTransform;

use crate::{Rend3Handle, Rend3};

pub fn transform_object(
    rend3: Rend3,
    query: Query<(&Rend3Handle<Object>, &GlobalTransform), Changed<GlobalTransform>>
) {
    for (object, transform) in query.iter() {
        rend3.set_object_transform(object, transform.compute_matrix());
    }
}