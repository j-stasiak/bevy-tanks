pub mod collider;
pub mod macros;
pub mod shooting;
pub mod tank;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shooting::ShootingPlugin;
use tank::TankPlugin;

#[derive(Component)]
pub struct GameCamera;

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tank game".into(),
                        resolution: (1280.0, 960.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TankPlugin)
        .add_plugins(ShootingPlugin)
        .add_systems(Startup, init_camera)
        .run();
}
