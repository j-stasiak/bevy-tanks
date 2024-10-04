use bevy::prelude::*;
use bevy_lunex::prelude::*;

#[derive(Component)]
pub struct PrimaryButton {
    pub file: &'static str,
}

#[derive(Component, Debug, Default, Clone, PartialEq)]
struct PrimaryButtonUi;

fn build_component(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &PrimaryButton), Added<PrimaryButton>>,
) {
    let font_handler: Handle<Font> = asset_server.load("fonts/AldotheApache.ttf");
    for (entity, button_source) in &query {
        // This will create a private sandboxed UiTree within the entity just for the button
        commands
            .entity(entity)
            .insert(UiTreeBundle::<PrimaryButtonUi>::from(UiTree::new2d(
                "PrimaryButton",
            )))
            .with_children(|ui| {
                let image = ui
                    .spawn((
                        UiLink::<PrimaryButtonUi>::path("Control/Image"),
                        // Add layout
                        UiLayout::window_full().pack::<Base>(),
                        UiImage2dBundle::from(asset_server.load::<Image>(button_source.file)),
                        PickableBundle::default(),
                        UiLayout::window_full().x(Rl(10.0)).pack::<Hover>(),
                        // Required to tween between states
                        UiLayoutController::default(),
                        UiClickEmitter::new(entity),
                        UiAnimator::<Hover>::new()
                            .forward_speed(5.0)
                            .backward_speed(7.0),
                    ))
                    .id();

                // ui.spawn((
                //     // Link this widget
                //     UiLink::<PrimaryButtonUi>::path("Control"),
                //     // Add layout
                //     UiLayout::window_full().pack::<Base>(),
                //     // Make this spacial & clickable entity
                //     UiZoneBundle::default(),
                //     // This is required to control our hover animation
                //     // This will pipe this hover data to the specified entities
                //     UiAnimatorPipe::<Hover>::new(vec![image]),
                //     // If we click on this hover zone, it will emmit UiClick event from parent entity
                // ));
            });
    }
}

/// Plugin adding all our logic
pub struct PrimaryButtonPlugin;
impl Plugin for PrimaryButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiDebugPlugin::<PrimaryButtonUi>::new())
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiGenericPlugins::<PrimaryButtonUi>::new())
            // Add general systems
            .add_systems(Update, build_component.before(UiSystems::Compute));
    }
}
