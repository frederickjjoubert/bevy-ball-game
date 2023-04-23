use bevy::prelude::*;

use crate::{
    bundles::{button_bundle, text_bundle},
    game::{
        score::resources::Score,
        ui::game_over_menu::{
            components::{FinalScoreText, GameOverMenu, MainMenuButton, QuitButton, RestartButton},
            styles::{
                BACKGROUND_COLOR, GAME_OVER_MENU_CONTAINER_STYLE, GAME_OVER_MENU_STYLE,
                NORMAL_BUTTON,
            },
        },
    },
};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    build_game_over_menu(&mut commands, &asset_server, score);
}
pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

pub fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    score: Res<Score>,
) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            NodeBundle {
                style: GAME_OVER_MENU_STYLE,
                z_index: ZIndex::Local(2),
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: GAME_OVER_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(text_bundle("Game Over", &asset_server, 64.0));
                    parent.spawn((
                        text_bundle("Your final score was", asset_server, 32.0),
                        FinalScoreText {},
                    ));
                    parent.spawn(text_bundle(&score.value.to_string(), asset_server, 64.0));
                    parent
                        .spawn((button_bundle(200.0, 80.0, NORMAL_BUTTON), RestartButton {}))
                        .with_children(|parent| {
                            parent.spawn(text_bundle("Restart", asset_server, 32.0));
                        });
                    parent
                        .spawn((button_bundle(200.0, 80.0, NORMAL_BUTTON), MainMenuButton {}))
                        .with_children(|parent| {
                            parent.spawn(text_bundle("Main Menu", asset_server, 32.0));
                        });
                    parent
                        .spawn((button_bundle(200.0, 80.0, NORMAL_BUTTON), QuitButton {}))
                        .with_children(|parent| {
                            parent.spawn(text_bundle("Quit", asset_server, 32.0));
                        });
                });
        })
        .id();
    return game_over_menu_entity;
}
