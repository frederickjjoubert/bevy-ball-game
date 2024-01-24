use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::player::components::Player;
use crate::game::target::components::Target;

use crate::events::GameOver;
use crate::game::enemy::components::*;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::score::resources::*;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;

use super::components::Projectile;

pub fn spawn_projectile(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // player_query: Query<(Entity, &Transform), With<Player>>,
    // target_query: Query<&Transform, With<Target>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_white_large.png"),
            ..default()
        },
        Projectile,
    ));
}

pub fn despawn_projectile(mut commands: Commands, projectile_query: Query<Entity, With<Projectile>>) {
    if let Ok(projectile_entity) = projectile_query.get_single() {
        commands.entity(projectile_entity).despawn();
    }
}
