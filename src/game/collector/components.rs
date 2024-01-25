use bevy::prelude::*;

#[derive(Component)]

pub struct Collector {
    pub velocity: Vec2,
}

#[derive(Event)]
pub struct CollectorSpawnEvent {
    pub spawn_pos: Transform,
}