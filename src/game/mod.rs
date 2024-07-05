pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::GameUIPlugin;

use bevy::prelude::*;

use crate::events::GameOver;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .init_state::<SimulationState>()
            // OnEnter Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // My Plugins
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
                GameUIPlugin,
            ))
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
