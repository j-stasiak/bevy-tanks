pub mod components;

use bevy::{asset::AssetPath, prelude::*};
use components::{
    SpawnTankEvent, Tank, TankAssetsLoader, TankConfiguration, TankController, TankGun,
    TANK_COLORS, TANK_TYPES,
};

use crate::{
    camera::{CameraFollow, GameCamera},
    get_single_or_return,
    shooting::{ProjectileKind, ShotProjectileEvent},
    ApplicationState,
};

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankAssetsLoader {
            handles: vec![],
            loading_items: 0,
        });
        app.add_event::<SpawnTankEvent>();
        app.add_systems(OnEnter(ApplicationState::InGame), load_tank);
        app.add_systems(OnExit(ApplicationState::InGame), remove_tanks);
        app.add_systems(
            Update,
            (shoot_handler, spawn_tank_listener).run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            (rotate_gun_to_mouse, move_tank).run_if(in_state(ApplicationState::InGame)),
        );
    }
}

fn load_tank(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tank_hull_handle: Handle<Image> = asset_server.load("sprites/tanks/falcon/hull-blue.png");
    let tank_gun_handle: Handle<Image> = asset_server.load("sprites/tanks/falcon/gun-blue.png");

    commands
        .spawn((
            Name::new("Le tank"),
            CameraFollow,
            Tank {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -20.),
            },
            TankController,
            SpriteBundle {
                texture: tank_hull_handle,
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Le gun"),
                TankGun {
                    turning_speed: f32::to_radians(180.),
                },
                SpriteBundle {
                    texture: tank_gun_handle,
                    transform: Transform::from_xyz(0., 0., 2.),
                    ..default()
                },
            ));
        });
}

fn move_tank(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Tank), With<TankController>>,
) {
    for (mut transform, tank) in query.iter_mut() {
        let mut rotation_factor = 0.0;
        let mut movement_factor = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_factor += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_factor -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement_factor += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement_factor -= 1.0;
            rotation_factor *= -1.;
        }

        transform.rotate_z(rotation_factor * tank.turning_speed * time.delta_seconds());

        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = movement_factor * tank.speed * time.delta_seconds();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
    }
}

fn rotate_gun_to_mouse(
    time: Res<Time>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    tank_query: Query<(&Transform, &Tank), (With<Tank>, With<TankController>)>,
    mut gun_query: Query<(&mut Transform, &TankGun, &Parent), Without<Tank>>,
) {
    // Get the window and camera
    let window = get_single_or_return!(windows);
    let (camera, camera_transform) = get_single_or_return!(camera_query);

    // Get cursor position
    let cursor_position = match window.cursor_position() {
        Some(cursor) => cursor,
        None => return,
    };

    // Get world position of cursor
    let cursor_world_position = match camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        Some(position) => position,
        None => return,
    };

    // Iterate over guns and their parent tanks
    for (mut gun_transform, tank_gun, parent) in gun_query.iter_mut() {
        if let Ok((tank_transform, tank)) = tank_query.get(parent.get()) {
            // Calculate gun's world position
            let gun_mounting_point = tank.mounting_point.extend(0.);
            let gun_world_position = tank_transform.transform_point(gun_mounting_point);

            // Calculate direction from gun to cursor in world space
            let to_cursor = (cursor_world_position - gun_world_position.xy()).normalize();

            // Calculate the current forward direction of the gun in world space
            let gun_forward = tank_transform.rotation * gun_transform.rotation * Vec3::Y;

            // Calculate the angle between the current gun direction and the target direction
            let angle = gun_forward.xy().angle_between(to_cursor);

            // Determine rotation direction (clockwise or counter-clockwise)
            let rotation_direction = gun_forward.xy().perp_dot(to_cursor).signum();

            // Calculate rotation amount, clamped by turning speed
            let rotation_amount = rotation_direction
                * (tank_gun.turning_speed * time.delta_seconds()).min(angle.abs());

            gun_transform.rotate_around(gun_mounting_point, Quat::from_rotation_z(rotation_amount));
        }
    }
}

// should probably be moved to input plugin?
fn shoot_handler(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut event_writer: EventWriter<ShotProjectileEvent>,
    tank_gun_query: Query<&GlobalTransform, With<TankGun>>,
) {
    let transform = get_single_or_return!(tank_gun_query);

    if mouse_input.just_pressed(MouseButton::Left) {
        let Vec3 { x, y, .. } = transform.transform_point(Vec3::from((0., 128., 0.)));
        event_writer.send(ShotProjectileEvent {
            origin: Vec2::from((x, y)),
            rotation: transform.compute_transform().rotation,
            kind: ProjectileKind::Shell,
        });
    }
}

fn spawn_tank_listener(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<SpawnTankEvent>,
) {
    for event in event_reader.read() {
        let tank_type = event.r#type.get_folder_name();
        let TankConfiguration {
            speed,
            turning_speed,
            mounting_point,
            gun_turning_speed,
        } = event.r#type.get_configuration();
        let tank_color = event.color.get_folder_name();

        let tank_hull_handle: Handle<Image> =
            asset_server.load(format!("sprites/tanks/{tank_type}/hull-{tank_color}.png"));
        let tank_gun_handle: Handle<Image> =
            asset_server.load(format!("sprites/tanks/{tank_type}/gun-{tank_color}.png"));

        commands
            .spawn((
                Name::new("Le tank"),
                CameraFollow,
                Tank {
                    speed,
                    turning_speed,
                    mounting_point,
                },
                TankController,
                SpriteBundle {
                    texture: tank_hull_handle,
                    transform: Transform::from_translation(event.spawn_place.extend(0.)),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("Le gun"),
                    TankGun {
                        turning_speed: gun_turning_speed,
                    },
                    SpriteBundle {
                        texture: tank_gun_handle,
                        transform: Transform::from_xyz(0., 0., 2.),
                        ..default()
                    },
                ));
            });
    }
}

fn load_tanks(asset_server: Res<AssetServer>, mut tank_handlers: ResMut<TankAssetsLoader>) {
    for tank_type in TANK_TYPES {
        for color in TANK_COLORS {
            let gun_path: AssetPath = format!("sprites/tanks/{tank_type}/gun-{color}").into();
            let hull_path: AssetPath = format!("sprites/tanks/{tank_type}/hull-{color}").into();

            let gun_handler: Handle<Image> = asset_server.load(gun_path);
            let hull_handler: Handle<Image> = asset_server.load(hull_path);

            tank_handlers.handles.push(gun_handler.clone_weak());
            tank_handlers.handles.push(hull_handler.clone_weak());
        }
    }

    tank_handlers.loading_items += tank_handlers.handles.len() as i32;
}

fn remove_tanks(mut commands: Commands, query: Query<Entity, With<Tank>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
