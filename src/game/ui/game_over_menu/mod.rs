use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::interactive::{
    interact_with_main_menu_button, interact_with_quit_button, interact_with_restart_button,
};
use systems::layout::{despawn_game_over_menu, spawn_game_over_menu};

use crate::AppState;
pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_systems(
                (
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
