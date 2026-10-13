mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::game::SimulationState;

use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            // Systems
            .add_systems(
                Update,
                update_button_colors.run_if(in_state(SimulationState::Paused)),
            )
            // Button Observers
            .add_observer(interact_with_resume_button)
            .add_observer(interact_with_main_menu_button)
            .add_observer(interact_with_quit_button)
            // OnExit Systems
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
