mod game_over_menu;
mod hud;
mod pause_menu;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((HudPlugin, PauseMenuPlugin, GameOverMenuPlugin));
    }
}
