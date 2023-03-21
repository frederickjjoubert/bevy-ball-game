use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu);
    }
}

pub fn main_menu() {
    println!("You are on the main menu.");
}
