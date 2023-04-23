use bevy::prelude::*;

use crate::{
    bundles::text_bundle,
    game::ui::hud::{
        components::{EnemyText, Hud, ScoreText},
        styles::{hub_card_node_bundle, HUB_STYLE, IMAGE_STYLE},
    },
};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}
pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUB_STYLE,
                ..default()
            },
            Hud {},
        ))
        .with_children(|parent| {
            parent
                .spawn(hub_card_node_bundle(32.0, 0.0))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/star.png").into(),
                        ..default()
                    });
                    parent.spawn((text_bundle("0", &asset_server, 32.0), ScoreText {}));
                });
            parent
                .spawn(hub_card_node_bundle(0.0, 32.0))
                .with_children(|parent| {
                    parent.spawn((text_bundle("0", &asset_server, 32.0), EnemyText {}));
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });
        })
        .id();
    return hud_entity;
}

pub fn despawn_hud(mut commands: Commands, hub_query: Query<Entity, With<Hud>>) {
    if let Ok(hud_entity) = hub_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}
