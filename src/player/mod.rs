mod components;
mod systems;

use systems::*;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_startup_system(spawn_player)
            // Systems
            .add_system(confine_player_movement)
            .add_system(player_movement)
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
