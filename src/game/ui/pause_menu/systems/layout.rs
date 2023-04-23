use bevy::prelude::*;

use crate::{
    bundles::{button_bundle, text_bundle},
    game::ui::pause_menu::{
        components::{MainMenuButton, PauseMenu, QuitButton, ResumeButton},
        styles::{BACKGROUND_COLOR, NORMAL_BUTTON, PAUSE_MENU_CONTAINER_STYLE, PAUSE_MENU_STYLE},
    },
};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                z_index: ZIndex::Local(1),
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: PAUSE_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(text_bundle("Pause Menu", &asset_server, 64.0));
                    parent
                        .spawn((button_bundle(200.0, 80.0, NORMAL_BUTTON), ResumeButton {}))
                        .with_children(|parent| {
                            parent.spawn(text_bundle("Resume", &asset_server, 32.0));
                        });
                    parent
                        .spawn((button_bundle(200.0, 80.0, NORMAL_BUTTON), MainMenuButton {}))
                        .with_children(|parent| {
                            parent.spawn(text_bundle("Main Menu", &asset_server, 32.0));
                        });
                    parent
                        .spawn((button_bundle(200.0, 80.0, NORMAL_BUTTON), QuitButton {}))
                        .with_children(|parent| {
                            parent.spawn(text_bundle("Quit", &asset_server, 32.0));
                        });
                });
        })
        .id();
    return pause_menu_entity;
}
