pub mod components;
pub mod debri;
pub mod enemy;
pub mod player;
mod projectile;
pub mod score;
pub mod star;
pub mod collector;
mod systems;
mod target;
mod ui;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use score::ScorePlugin;
use collector::CollectorPlugin;
use star::StarPlugin;
use systems::*;
use target::TargetPlugin;
use ui::GameUIPlugin;

use bevy::prelude::*;

use crate::events::GameOver;
use crate::AppState;

use self::debri::DebriPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // OnEnter Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // My Plugins
            .add_plugins((
                // EnemyPlugin,
                ProjectilePlugin,
                TargetPlugin,
                PlayerPlugin,
                ScorePlugin,
                DebriPlugin,
                CollectorPlugin,
                // StarPlugin,
                GameUIPlugin,
            ))
            // Systems
            .add_systems(
                FixedUpdate,
                (toggle_simulation).run_if(in_state(AppState::Game)),
            )
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
