use super::ui_component::UiComponent;
use bevy::prelude::*;

#[derive(Component)]
pub struct PrimaryButton {
    pub text: &'static str,
    pub on_click: fn() -> (),
}

impl UiComponent for PrimaryButton {
    fn create(&self, parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
        let font_handler: Handle<Font> = asset_server.load("fonts/AldotheApache.ttf");

        parent
            .spawn(ButtonBundle {
                image: UiImage {
                    texture: asset_server.load("ui/Start_BTN.png"),
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Start",
                    TextStyle {
                        font: font_handler.clone(),
                        font_size: 60.,
                        color: Color::WHITE,
                    },
                ));
            });
    }
}
