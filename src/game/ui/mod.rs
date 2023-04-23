use bevy::prelude::*;

use self::game_over_menu::GameOverMenuPlugin;
use self::hud::HudPlugin;
use self::pause_menu::PauseMenuPlugin;

mod game_over_menu;
mod hud;
mod pause_menu;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HudPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(GameOverMenuPlugin);
    }
}
