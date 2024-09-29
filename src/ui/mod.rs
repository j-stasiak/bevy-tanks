use bevy::prelude::*;
use ui_component::UiComponent;

pub mod primary_button;
pub mod ui_component;

pub fn spawn_component(
    component: impl UiComponent,
    parent: &mut ChildBuilder<'_>,
    asset_server: &Res<AssetServer>,
) {
    component.create(parent, asset_server);
}
