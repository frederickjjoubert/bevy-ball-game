mod components;
mod styles;
mod systems;

use systems::layout::*;

use crate::game::ui::hud::systems::updates::{update_enemy_text, update_score_text};
use crate::AppState;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems((update_score_text, update_enemy_text).in_set(OnUpdate(AppState::Game)))
            // OnExit Systems
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
