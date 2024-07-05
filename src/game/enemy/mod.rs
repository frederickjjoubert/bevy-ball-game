pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

use super::SimulationState;

pub const ENEMY_SIZE: f32 = 64.0; // The enemy sprite is 64x64 pixels.
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Startup Systems
            // .add_startup_system(spawn_enemies)
            // Enter State Systems
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            // Systems
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
