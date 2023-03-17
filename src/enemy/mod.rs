pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use bevy::prelude::*;

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
            .add_startup_system(spawn_enemies)
            // Systems
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement)
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_enemies_over_time);
    }
}
