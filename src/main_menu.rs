use bevy::prelude::*;
use bevy_lunex::{prelude::*, SourceFromCamera, UiTreeBundle};

// use crate::ApplicationState;

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MainMenuRoute;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, build_route.before(UiSystems::Compute));
    }
}

fn build_route(mut commands: Commands, assets: Res<AssetServer>, query: Query<Entity, Added<MainMenuRoute>>) {
    for route_entity in &query {
        commands.entity(route_entity).insert(
            SpatialBundle::default(),
        ).with_children(|route| {
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new2d("MainMenu")),
                SourceFromCamera,
            )).with_children(|ui| {
                 let root = UiLink::<MainUi>::path("Root");  // Here we can define the name of the node

                ui.spawn((
                    root.clone(),                           // Here we add the link
                    UiLayout::window_full().pack::<Base>(),         // This is where we define layout
                ));

                ui.spawn((
                    root.add("Background"),                           // Here we add the link
                    UiLayout::solid().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack::<Base>(),         // This is where we define layout
                    UiImage2dBundle::from(assets.load("ui/background.png")),
                ));
            });
        });
    }
}

fn show_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handler: Handle<Font> = asset_server.load("fonts/AldotheApache.ttf");

    commands
        .spawn((
            // This makes the UI entity able to receive camera data
            SourceFromCamera,
            // This is our UI system
            UiTreeBundle::<MainUi>::from(UiTree::new2d("MainMenu")),
        ))
        .with_children(|ui| {
            ui.spawn((
                // Link the entity
                UiLink::<MainUi>::path("Root"),
                // Specify UI layout
                UiLayout::window_full()
                    .pos(Ab(20.0))
                    .size(Rl(100.0) - Ab(40.0))
                    .pack::<Base>(),
            ));

            ui.spawn((
                // Link the entity
                UiLink::<MainUi>::path("Root/Start_Button"),
                // Specify UI layout
                UiLayout::window().pos(Rl((50.0, 50.0))).size((Rh(45.0), Rl(60.0))).pack::<Base>(),
                // Add image to the entity
                UiImage2dBundle::from(asset_server.load("ui/Start_BTN.png")),
            ));
        });
}
