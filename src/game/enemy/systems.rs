use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Transform::from_xyz(random_x, random_y, 0.0),
            Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
            Enemy::new(random::<f32>(), random::<f32>()),
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn confine_enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        let mut direction_changed = false;

        // Bound the enemy x position
        if translation.x < x_min {
            translation.x = x_min;
            enemy.direction.x *= -1.0;
            direction_changed = true;
        } else if translation.x > x_max {
            translation.x = x_max;
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        // Bound the enemy y position
        if translation.y < y_min {
            translation.y = y_min;
            enemy.direction.y *= -1.0;
            direction_changed = true;
        } else if translation.y > y_max {
            translation.y = y_max;
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        transform.translation = translation;

        if direction_changed {
            // Randomly play one of the two sound effects.
            let sound_effect = if random::<f32>() > 0.5 {
                asset_server.load("audio/pluck_001.ogg")
            } else {
                asset_server.load("audio/pluck_002.ogg")
            };
            commands.spawn((AudioPlayer::new(sound_effect), PlaybackSettings::DESPAWN));
        }
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());

    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Transform::from_xyz(random_x, random_y, 0.0),
            Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
            Enemy::new(random::<f32>(), random::<f32>()),
        ));
    }
}
