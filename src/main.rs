// Disable console on Windows for non-dev builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // My Plugins
        .add_plugins((MainMenuPlugin, GamePlugin))
        // Startup Systems
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(
            Update,
            (
                transition_to_game_state,
                transition_to_main_menu_state,
                exit_game,
                handle_game_over,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
