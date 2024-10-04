use bevy::prelude::*;
use bevy_lunex::{prelude::*, SourceFromCamera, UiTreeBundle};

use crate::{ui::primary_button::PrimaryButton, ApplicationState};

#[derive(Component, Clone, PartialEq)]
enum ButtonType {
    Start,
    Options,
    Exit,
}

impl ButtonType {
    fn file(&self) -> &'static str {
        match self {
            ButtonType::Start => "ui/Start_BTN.png",
            ButtonType::Options => "ui/Start_BTN.png",
            ButtonType::Exit => "ui/Exit_BTN.png",
        }
    }

    fn str(&self) -> &'static str {
        match self {
            ButtonType::Start => "Start",
            ButtonType::Options => "Options",
            ButtonType::Exit => "Exit",
        }
    }
}

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MainMenuRoute;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            build_route
                .before(UiSystems::Compute)
                .run_if(in_state(ApplicationState::MainMenu)),
        );
        app.add_systems(Update, button_click_system);
    }
}

fn build_route(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<MainMenuRoute>>,
) {
    for route_entity in &query {
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new2d("MainMenu")),
                        SourceFromCamera,
                    ))
                    .with_children(|ui| {
                        let root = UiLink::<MainUi>::path("Root"); // Here we can define the name of the node

                        ui.spawn((
                            root.clone(),                           // Here we add the link
                            UiLayout::window_full().pack::<Base>(), // This is where we define layout
                        ));

                        ui.spawn((
                            root.add("Background"), // Here we add the link
                            UiLayout::window_full().pack::<Base>(),
                            UiImage2dBundle {
                                texture: assets.load("ui/background.png"),
                                ..default()
                            },
                        ));

                        let buttons = root.add("Buttons");

                        ui.spawn((
                            buttons.clone(),
                            UiLayout::window()
                                .pos((Rl(50.) - Ab(134.), Rl(25.)))
                                .size((Ab(268.), Rl(50.)))
                                .pack::<Base>(),
                        ));

                        let button_types =
                            [ButtonType::Start, ButtonType::Options, ButtonType::Exit];

                        let offset_step = 1. / (button_types.len() - 1) as f32;

                        for (i, button_type) in button_types.iter().enumerate() {
                            // (...) * 2 - 1 to bring it into range of -1..1
                            let align_y = (i as f32 * offset_step) * 2. - 1.;
                            ui.spawn((
                                buttons.add(button_type.str()),
                                UiLayout::solid()
                                    .size((268., 128.))
                                    .scaling(Scaling::Fit)
                                    .align_y(align_y)
                                    .pack::<Base>(),
                                button_type.clone(),
                                PrimaryButton {
                                    file: button_type.file(),
                                },
                            ));
                        }
                    });
            });
    }
}

fn button_click_system(
    mut commands: Commands,
    mut events: EventReader<UiClickEvent>,
    query: Query<&ButtonType>,
    mut route_entity_query: Query<Entity, (With<MainMenuRoute>, Without<PrimaryButton>)>,
    mut next_app: ResMut<NextState<ApplicationState>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    // Iterate over all events
    for event in events.read() {
        // Get our entity
        if let Ok(button) = query.get(event.target) {
            match *button {
                ButtonType::Start => {
                    commands
                        .entity(route_entity_query.single_mut())
                        .despawn_recursive();
                    next_app.set(ApplicationState::InGame);
                }
                ButtonType::Options => todo!(),
                ButtonType::Exit => {
                    app_exit_events.send(AppExit::Success);
                }
            };
        }
    }
}
