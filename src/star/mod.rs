use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_stars_over_time);
    }
}
