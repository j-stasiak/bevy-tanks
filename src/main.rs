pub mod camera;
pub mod collider;
pub mod macros;
pub mod shooting;
pub mod tank;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use shooting::ShootingPlugin;
use tank::TankPlugin;

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
        .add_plugins(CameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TankPlugin)
        .add_plugins(ShootingPlugin)
        .run();
}
