pub mod components;
mod systems;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;
use systems::*;

pub const BUILDING_SIZE: f32 = 30.0;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(StartUp, (spawn_building,))
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_building);
    }
}
