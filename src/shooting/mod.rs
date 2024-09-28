use bevy::prelude::*;

use crate::collider::Collidable;

const BASE_BULLET_SPEED_MULTIPLIER: i32 = 25;

pub enum ProjectileKind {
    Shell,
    Bullet,
}

#[derive(Event)]
pub struct ShotProjectileEvent {
    pub origin: Vec2,
    pub rotation: Quat,
    pub kind: ProjectileKind,
}

#[derive(Event)]
pub struct ProjectileHitEvent {
    projectile_id: Entity,
    entities_hit: [Entity],
}

#[derive(Component)]
struct Projectile {
    speed: f32,
    damage: f32,
}

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShotProjectileEvent>();
        app.add_systems(Update, spawn_projectile);
        app.add_systems(
            FixedUpdate,
            (despawn_projectile, render_projectiles).chain(),
        );
    }
}

fn spawn_projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<ShotProjectileEvent>,
) {
    for event in event_reader.read() {
        let (speed, damage): (f32, f32) = match event.kind {
            ProjectileKind::Shell => (100., 150.),
            ProjectileKind::Bullet => (250., 25.),
        };

        commands.spawn((
            Projectile { speed, damage },
            Collidable,
            SpriteBundle {
                transform: Transform {
                    rotation: event.rotation,
                    translation: event.origin.extend(2.),
                    ..default()
                },
                texture: asset_server.load("sprites/Sniper_Shell.png"), // TODO: Change later to support multiple bullet textures depending on type
                ..default()
            },
        ));
    }
}

fn render_projectiles(
    time: Res<Time>,
    mut projectile_query: Query<(&mut Transform, &Projectile), With<Projectile>>,
) {
    for (mut transform, projectile) in &mut projectile_query {
        let movement_distance =
            BASE_BULLET_SPEED_MULTIPLIER as f32 * projectile.speed * time.delta_seconds();
        let translation_delta = (transform.rotation * Vec3::Y) * movement_distance;

        transform.translation += translation_delta;
    }
}

// normally should be used with event - ProjectileHitEvent
// but for now we check only boundaries of screen
fn despawn_projectile(
    mut commands: Commands,
    projectile_query: Query<(&Transform, Entity), With<Projectile>>,
) {
    for (transform, entity) in &projectile_query {
        let Vec3 { x, y, .. } = transform.translation;
        if x < -1280. || x > 1280. || y < -960. || y > 960. {
            commands.entity(entity).despawn_recursive();
        }
    }
}
