pub mod components;
mod systems;

use std::time::Duration;

use systems::*;

use super::{components::Collider, SimulationState};
use crate::AppState;

use bevy::prelude::*;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};

pub const DEBRI_SIZE: f32 = 3.0;

pub struct DebriPlugin;

impl Plugin for DebriPlugin {
    fn build(&self, app: &mut App) {
        app
            //Plugins
            .add_plugins(
                AutomaticUpdate::<Collider>::new()
                    .with_spatial_ds(SpatialStructure::KDTree2)
                    .with_frequency(Duration::from_secs(1))
                    .with_transform(TransformMode::Transform),
            )
            // Events
            .add_event::<components::SpawnDebri>()
            // Systems
            .add_systems(
                FixedUpdate,
                (debri_movement, spawn_debri)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_debri);
    }
}
