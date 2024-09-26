use bevy::prelude::*;

pub const TANK_COLORS: [&'static str; 1] = ["green"];
pub const TANK_TYPE: [&'static str; 1] = ["default"];

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
