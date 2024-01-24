pub mod components;
mod systems;

use systems::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub const DEBRI_SIZE: f32 = 3.0;

pub struct DebriPlugin;

impl Plugin for DebriPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(
                FixedUpdate,
                (debri_movement,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_debri);
    }
}
