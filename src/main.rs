pub mod camera;
pub mod collider;
pub mod macros;
pub mod main_menu;
pub mod shooting;
pub mod tank;
pub mod ui;

use bevy::prelude::*;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_lunex::prelude::MainUi;
use bevy_lunex::UiDebugPlugin;
use bevy_lunex::UiDefaultPlugins;
use camera::CameraPlugin;
use main_menu::MainMenuPlugin;
use main_menu::MainMenuRoute;
use shooting::ShootingPlugin;
use tank::TankPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ApplicationState {
    LoadingScreen,
    MainMenu,
    InGame,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tank game".into(),
                    resolution: (1280.0, 960.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            // Required only for older graphics cards that don't support Vulcan that well
            .set(RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::GL),
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
    .add_plugins(UiDefaultPlugins)
    .add_plugins(MainMenuPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(TankPlugin)
    .add_plugins(ShootingPlugin)
    .add_plugins(UiDebugPlugin::<MainUi>::new())
    .insert_state(ApplicationState::MainMenu)
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(MainMenuRoute);
}