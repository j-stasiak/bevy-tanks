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
            TankType::Lion => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
            TankType::Bear => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
            TankType::Shark => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
            TankType::Ant => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
            TankType::Mantis => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
            TankType::Gorilla => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
            TankType::Falcon => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
            TankType::Dragon => TankConfiguration {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
                gun_turning_speed: f32::to_radians(180.),
            },
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
