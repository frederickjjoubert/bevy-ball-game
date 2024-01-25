use bevy::prelude::*;

use crate::game::ui::spawn_toolbar::components::{BuldingType, DefaultButton, SpawnToolbar};
use crate::game::ui::spawn_toolbar::styles::*;

pub fn spawn_spawn_toolbar(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Pause Menu");
    build_spawn_toolbar(&mut commands, &asset_server);
}

pub fn despawn_spawn_toolbar(
    mut commands: Commands,
    spawn_toolbar_query: Query<Entity, With<SpawnToolbar>>,
) {
    if let Ok(spawn_toolbar_entity) = spawn_toolbar_query.get_single() {
        commands.entity(spawn_toolbar_entity).despawn_recursive();
    }
}

// System Piping Example
pub fn build_spawn_toolbar(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let spawn_toolbar_entity = commands
        .spawn((
            NodeBundle {
                style: SPAWN_TOOLBAR_STYLE,
                z_index: ZIndex::Local(1), // See Ref. 1
                ..default()
            },
            SpawnToolbar {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: SPAWN_TOOLBAR_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            DefaultButton::Shooter,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Shooter",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            DefaultButton::Collector,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Collector",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            DefaultButton::Building(BuldingType::Collector),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Stash",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    spawn_toolbar_entity
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
