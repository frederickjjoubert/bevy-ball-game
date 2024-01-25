mod game_over_menu;
mod hud;
mod pause_menu;
mod spawn_toolbar;

use bevy::prelude::*;
use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;
use spawn_toolbar::SpawnToolbarPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((
                HudPlugin,
                PauseMenuPlugin,
                GameOverMenuPlugin,
                SpawnToolbarPlugin,
            ));
    }
}
