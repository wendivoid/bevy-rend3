use bevy_ecs::prelude::*;
use rend3::types::ResourceHandle;

#[derive(Component)]
pub struct Rend3Handle<T>(pub ResourceHandle<T>);

impl <T>From<ResourceHandle<T>> for Rend3Handle<T> {
    fn from(f: ResourceHandle<T>) -> Rend3Handle<T> {
        Rend3Handle(f)
    }
}