use bevy::prelude::*;

use crate::game::ui::hud::components::*;
use crate::game::ui::hud::styles::*;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((Visibility::default(), HUD_NODE.clone(), HUD {}))
        .with_children(|parent| {
            // LHS
            parent
                .spawn((LHS_NODE.clone(), BackgroundColor(BACKGROUND_COLOR)))
                .with_children(|parent| {
                    // Star Image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/star.png")),
                        IMAGE_NODE.clone(),
                    ));
                    // Score Text
                    parent.spawn((
                        Text("0".to_string()),
                        get_text_style(asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                        ScoreText {},
                    ));
                });
            // RHS
            parent
                .spawn((RHS_NODE.clone(), BackgroundColor(BACKGROUND_COLOR)))
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        Text("0".to_string()),
                        get_text_style(asset_server),
                        TextLayout::new_with_justify(JustifyText::Center),
                        EnemyText {},
                    ));
                    // Enemy Image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                        IMAGE_NODE.clone(),
                    ));
                });
        })
        .id();

    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
