pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use bevy::prelude::*;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(OnEnter(AppState::Game), spawn_target)
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_target);
    }
}
