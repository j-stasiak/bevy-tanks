pub mod components;

use bevy::prelude::*;
use components::{Tank, TankController, TankGun};

use crate::{
    get_single_or_return,
    shooting::{ProjectileKind, ShotProjectileEvent},
    GameCamera,
};

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_tank);
        app.add_systems(Update, shoot_handler);
        app.add_systems(FixedUpdate, (rotate_gun_to_mouse, move_tank));
    }
}

fn load_tank(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tank_hull_handle: Handle<Image> = asset_server.load("sprites/tank-hull-green-default.png");
    let tank_gun_handle: Handle<Image> = asset_server.load("sprites/tank-gun-green-default.png");

    commands
        .spawn((
            Name::new("Le tank"),
            Tank {
                speed: 400.,
                turning_speed: f32::to_radians(90.),
                mounting_point: Vec2::new(0., -40.),
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
