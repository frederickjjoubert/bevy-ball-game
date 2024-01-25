mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;
use crate::AppState;

use bevy::prelude::*;

pub struct SpawnToolbarPlugin;

impl Plugin for SpawnToolbarPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(OnEnter(AppState::Game), spawn_spawn_toolbar)
            // Systems
            .add_systems(
                Update,
                (interact_with_button).run_if(in_state(AppState::Game)),
            )
            // OnExit Systems
            .add_systems(OnExit(AppState::Game), despawn_spawn_toolbar);
    }
}
