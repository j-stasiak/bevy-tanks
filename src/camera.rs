use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{get_single_mut_or_return, get_single_or_return};

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct CameraFollow;

#[derive(Component)]
pub struct Zoom;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
        app.add_systems(Update, (follow_entity, zoom_camera));
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera, Zoom));
}

fn follow_entity(
    mut camera_transform_query: Query<&mut Transform, With<GameCamera>>,
    transform_query: Query<&Transform, (With<CameraFollow>, Without<GameCamera>)>,
) {
    let mut camera_transform = get_single_mut_or_return!(camera_transform_query);
    let transform = get_single_or_return!(transform_query);

    camera_transform.translation = transform.translation;
}

fn zoom_camera(
    mut camera_transform_query: Query<&mut OrthographicProjection, With<Zoom>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let mut camera_transform = get_single_mut_or_return!(camera_transform_query);
    use bevy::input::mouse::MouseScrollUnit;

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let mut log_scale = camera_transform.scale.ln();
                log_scale -= 0.1 * ev.y;
                camera_transform.scale = log_scale.exp().clamp(0.4, 1.6);
            }
            MouseScrollUnit::Pixel => (),
        }
    }
}
