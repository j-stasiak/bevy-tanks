use bevy::{
    asset::AssetServer,
    prelude::{ChildBuilder, Res},
};

pub trait UiComponent {
    fn create(&self, parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>);
}
