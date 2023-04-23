use bevy::prelude::*;

use crate::game::{
    enemy::components::Enemy,
    score::resources::Score,
    ui::hud::components::{EnemyText, ScoreText},
};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}
pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let enemies = enemy_query.iter().count();
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!("{}", enemies.to_string());
    }
}
