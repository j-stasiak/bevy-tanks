use bevy::prelude::*;

#[derive(Component)]
pub struct Collidable;

#[derive(Event)]
pub struct HitColliderEvent {
    origin: Entity,
    target_hit: Entity,
}

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
