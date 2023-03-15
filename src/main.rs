pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
