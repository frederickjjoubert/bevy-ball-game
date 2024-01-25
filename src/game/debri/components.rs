use bevy::prelude::*;
use bevy_spatial::kdtree::KDTree2;

use crate::game::components::{Collider, Position};
#[derive(Component)]

pub struct Debri {
    pub velocity: Vec2,
    pub time_alive: f32,
    pub start_deceleration: bool,
}

pub type NNTree = KDTree2<Collider>;

#[derive(Event)]
pub struct SpawnDebri {
    pub direction: Vec2,
    pub position: Position,
}
