pub mod components;
pub mod resources;
mod systems;

use systems::*;
use resources::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<ProjectileSpawnTimer>()
            // Systems
            .add_systems(
                FixedUpdate,
                (
                    projectile_hit_target,
                    projectile_movement,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (
                    spawn_projectile_timer,
                    tick_projectile_spawn_timer,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_projectile);
    }
}
