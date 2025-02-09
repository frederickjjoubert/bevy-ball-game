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
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game).and(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
