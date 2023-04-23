use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::layout::spawn_hud;

use crate::AppState;

use self::systems::{
    interactions::{update_enemy_text, update_score_text},
    layout::despawn_hud,
};
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            .add_systems((update_score_text, update_enemy_text).in_set(OnUpdate(AppState::Game)))
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
