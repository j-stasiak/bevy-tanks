use bevy::prelude::*;

pub const TANK_COLORS: [&'static str; 4] = ["green", "brown", "turquoise", "blue"];
pub const TANK_TYPES: [&'static str; 8] = [
    "lion", "bear", "shark", "ant", "mantis", "gorilla", "falcon", "dragon",
];

pub struct TankConfiguration {
    pub speed: f32,
    pub turning_speed: f32,
    pub mounting_point: Vec2,
    pub gun_turning_speed: f32,
}

impl Into<TankConfiguration> for (f32, f32, Vec2, f32) {
    fn into(self) -> TankConfiguration {
        TankConfiguration {
            speed: self.0,
            turning_speed: self.1,
            mounting_point: self.2,
            gun_turning_speed: self.3,
        }
    }
}

pub enum TankColor {
    Green,
    Brown,
    Turquoise,
    Blue,
}

pub enum TankType {
    Lion,
    Bear,
    Shark,
    Ant,
    Mantis,
    Gorilla,
    Falcon,
    Dragon,
}

impl TankType {
    pub fn get_folder_name(&self) -> &'static str {
        match self {
            TankType::Lion => "lion",
            TankType::Bear => "bear",
            TankType::Shark => "shark",
            TankType::Ant => "ant",
            TankType::Mantis => "mantis",
            TankType::Gorilla => "gorilla",
            TankType::Falcon => "falcon",
            TankType::Dragon => "dragon",
        }
    }

    pub fn get_configuration(&self) -> TankConfiguration {
        match self {
            TankType::Lion => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
            TankType::Bear => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
            TankType::Shark => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
            TankType::Ant => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
            TankType::Mantis => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
            TankType::Gorilla => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
            TankType::Falcon => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
            TankType::Dragon => (
                400.,
                f32::to_radians(90.),
                Vec2::new(0., -40.),
                f32::to_radians(180.),
            )
                .into(),
        }
    }
}

impl TankColor {
    pub fn get_folder_name(&self) -> &'static str {
        match self {
            TankColor::Green => "green",
            TankColor::Brown => "brown",
            TankColor::Turquoise => "turquoise",
            TankColor::Blue => "blue",
        }
    }
}

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
pub struct TankAssetsLoader {
    pub handles: Vec<Handle<Image>>,
    pub loading_items: i32,
}

#[derive(Event)]
pub struct SpawnTankEvent {
    pub spawn_place: Vec2,
    pub r#type: TankType,
    pub color: TankColor,
}
