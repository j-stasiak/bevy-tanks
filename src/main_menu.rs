use bevy::prelude::*;

use crate::ApplicationState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            show_main_menu.run_if(in_state(ApplicationState::MainMenu)),
        );
    }
}

fn show_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handler: Handle<Font> = asset_server.load("fonts/AldotheApache.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                padding: UiRect {
                    bottom: Val::Percent(10.),
                    left: Val::Percent(10.),
                    right: Val::Percent(10.),
                    top: Val::Percent(10.),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // parent
            // .spawn(NodeBundle {
            //     style: Style {
            //         align_content: AlignContent::Center,
            //         justify_content: JustifyContent::Center,
            //         ..default()
            //     },
            //     ..default()
            // })
            // .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: font_handler.clone(),
                    font_size: 60.,
                    color: Color::WHITE,
                },
            ));
            // });
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settings",
                        TextStyle {
                            font: font_handler.clone(),
                            font_size: 60.,
                            color: Color::WHITE,
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: font_handler.clone(),
                            font_size: 60.,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}
