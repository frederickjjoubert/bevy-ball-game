use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::game::debri::{components::Debri, DEBRI_SIZE};
use crate::game::player::components::Player;
use crate::game::target::components::Target;

use crate::game::components::Collider;

use super::components::Projectile;
use super::resources::*;

// spawn a projectile at the player's position and move it towards the target
pub fn spawn_projectile(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    target_query: Query<&Transform, With<Target>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(target_transform) = target_query.get_single() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/projectile.png"),
                    ..default()
                },
                Projectile {
                    target: target_transform.translation,
                },
            ));
        }
    }
}

pub fn projectile_movement(
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in projectile_query.iter_mut() {
        let direction = projectile.target - transform.translation;
        let velocity = direction.normalize() * time.delta_seconds() * 500.0;
        transform.translation += velocity;
    }
}

pub fn despawn_projectile(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_projectile_spawn_timer(
    mut projectile_spawn_timer: ResMut<ProjectileSpawnTimer>,
    time: Res<Time>,
) {
    projectile_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_projectile_timer(
    mut commands: Commands,
    projectile_spawn_timer: Res<ProjectileSpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    target_query: Query<&Transform, With<Target>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if projectile_spawn_timer.timer.finished() {
        println!("spawn projectile");
        let target_transform = target_query.get_single().unwrap();
        for player_transform in player_query.iter() {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube { size: 1.0 }.into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_xyz(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        0.0,
                    ),
                    ..default()
                },
                Projectile {
                    target: target_transform.translation,
                },
            ));
        }
    }
}

pub fn projectile_hit_target(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    target_query: Query<&Transform, With<Target>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for (projectile_entity, projectile_transform) in projectile_query.iter_mut() {
        if let Ok(target_transform) = target_query.get_single() {
            let distance = projectile_transform
                .translation
                .distance(target_transform.translation);
            if distance < 10.0 {
                // Direction from target to projectile
                let hit_direction = projectile_transform.translation - target_transform.translation;

                // Negate and normalize the direction for debris velocity
                let debris_velocity = -hit_direction.normalize() * rng.gen_range(100.0..200.0);

                commands.entity(projectile_entity).despawn();
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(DEBRI_SIZE).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: Transform::from_xyz(
                            target_transform.translation.x,
                            target_transform.translation.y,
                            0.0,
                        ),
                        ..default()
                    },
                    Debri {
                        velocity: Vec2::new(debris_velocity.x, debris_velocity.y),
                        time_alive: 0.0,
                        start_deceleration: false,
                    },
                    Collider {
                        size: Vec2::new(DEBRI_SIZE, DEBRI_SIZE),
                    },
                ));
            }
        }
    }
}
