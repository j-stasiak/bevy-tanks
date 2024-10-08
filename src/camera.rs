use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_lunex::{prelude::MainUi, CursorBundle};

use crate::{get_single_mut_or_return, get_single_or_return, ApplicationState};

#[derive(Component)]
pub struct MainMenuCamera;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct CameraFollow;

#[derive(Component)]
pub struct Zoom;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::MainMenu), init_main_menu_camera);
        app.add_systems(OnEnter(ApplicationState::InGame), init_camera);
        app.add_systems(
            OnExit(ApplicationState::MainMenu),
            despawn_camera::<MainMenuCamera>,
        );
        app.add_systems(
            OnExit(ApplicationState::InGame),
            despawn_camera::<GameCamera>,
        );
        app.add_systems(
            Update,
            (follow_entity, zoom_camera).run_if(in_state(ApplicationState::InGame)),
        );
    }
}

fn init_main_menu_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1000.0),
                ..default()
            },
            MainUi,
            MainMenuCamera,
        ))
        .with_children(|parent| {
            parent.spawn(CursorBundle::default());
        });
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

fn despawn_camera<T: Component>(
    mut commands: Commands,
    camera_entity_query: Query<Entity, With<T>>,
) {
    let camera = get_single_or_return!(camera_entity_query);
    commands.entity(camera).despawn_recursive();
}
