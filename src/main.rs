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
use bevy_dev_tools::ui_debug_overlay::DebugUiPlugin;
use bevy_dev_tools::ui_debug_overlay::UiDebugOptions;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    .insert_state(ApplicationState::MainMenu)
    .add_systems(Startup, setup);

    #[cfg(feature = "bevy_dev_tools")]
    {
        app.add_plugins(DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(MainMenuRoute);
}

#[cfg(feature = "bevy_dev_tools")]
// The system that will enable/disable the debug outlines around the nodes
fn toggle_overlay(input: Res<ButtonInput<KeyCode>>, mut options: ResMut<UiDebugOptions>) {
    info_once!("The debug outlines are enabled, press Space to turn them on/off");
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug_overlay if disabled and disable if enabled
        options.toggle();
    }
}
