use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;

pub fn spawn_target(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(center_x, center_y, 0.0),
            texture: asset_server.load("sprites/ball_white_large.png"),
            ..default()
        },
        Target {},
    ));
}

pub fn despawn_target(mut commands: Commands, target_query: Query<Entity, With<Target>>) {
    for target_entity in target_query.iter() {
        commands.entity(target_entity).despawn();
    }
}
