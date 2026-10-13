mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;
use systems::updates::*;

use crate::AppState;
use bevy::prelude::*;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                (update_button_colors, update_final_score_text)
                    .run_if(in_state(AppState::GameOver)),
            )
            // Button Observers
            .add_observer(interact_with_restart_button)
            .add_observer(interact_with_main_menu_button)
            .add_observer(interact_with_quit_button)
            // // OnExit State Systems
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
