use bevy::prelude::*;

use crate::game::ui::game_over_menu::components::*;
use crate::game::ui::game_over_menu::styles::*;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            Visibility::default(),
            GAME_OVER_MENU_NODE,
            ZIndex(2), // See Ref. 1
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    GAME_OVER_MENU_CONTAINER_NODE,
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text("Game Over".to_string()),
                        get_title_text_style(asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    // Final Score Text
                    parent.spawn((
                        Text("Your final score was:".to_string()),
                        get_final_score_text_style(asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                        FinalScoreText {},
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            Button,
                            BUTTON_NODE,
                            BackgroundColor(NORMAL_BUTTON),
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text("Restart".to_string()),
                                get_button_text_style(asset_server),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            Button,
                            BUTTON_NODE,
                            BackgroundColor(NORMAL_BUTTON),
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text("Main Menu".to_string()),
                                get_button_text_style(asset_server),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                            // Quit Button
                            parent
                                .spawn((
                                    Button,
                                    BUTTON_NODE,
                                    BackgroundColor(NORMAL_BUTTON),
                                    QuitButton {},
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text("Quit".to_string()),
                                        get_button_text_style(asset_server),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                    ));
                                });
                        });
                });
        })
        .id();

    game_over_menu_entity
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
