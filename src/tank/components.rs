use bevy::prelude::*;

pub const TANK_COLORS: [&'static str; 4] = ["green", "brown", "turquoise", "blue"];
pub const TANK_TYPES: [&'static str; 8] = ["lion", "bear", "shark", "ant", "mantis", "gorilla", "falcon", "dragon"];
#[derive(Component)]
pub struct Tank {
    pub speed: f32,
    pub turning_speed: f32,
    pub mounting_point: Vec2,
}

#[derive(Component)]
pub struct TankGun {
    pub turning_speed: f32,
}

#[derive(Component)]
pub struct TankController;

#[derive(Resource)]
pub struct TankAssets {
    pub handles: Vec<Handle<Image>>,
    pub loading_items: i32,
}