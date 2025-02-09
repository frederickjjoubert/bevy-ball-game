use bevy::prelude::*;

use crate::game::ui::pause_menu::components::{
    MainMenuButton, PauseMenu, QuitButton, ResumeButton,
};
use crate::game::ui::pause_menu::styles::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Pause Menu");
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

// System Piping Example
pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            Visibility::default(),
            PAUSE_MENU_NODE.clone(),
            ZIndex(1), // See Ref. 1
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PAUSE_MENU_CONTAINER_NODE.clone(),
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text("Pause Menu".to_string()),
                        get_title_text_style(asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    // Resume Button
                    parent
                        .spawn((
                            Button,
                            BUTTON_NODE.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            ResumeButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text("Resume".to_string()),
                                get_button_text_style(asset_server),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            Button,
                            BUTTON_NODE.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text("Main Menu".to_string()),
                                get_button_text_style(asset_server),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                        });
                    // Quit Button
                    parent
                        .spawn((
                            Button,
                            BUTTON_NODE.clone(),
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
        })
        .id();

    pause_menu_entity
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
