use crate::PlayerAccountResource;
use bevy::prelude::*;
use bevy_ecs::prelude::*;
#[derive(Bundle)]
pub struct PlayerBundle {
    pub account: PlayerAccountResource,
    pub sprite: SpriteBundle,
}

#[derive(Default, Reflect, Component, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// type WorldStateQuery = Query<(&PlayerBundle )>;
