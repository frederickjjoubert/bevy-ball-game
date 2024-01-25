use bevy::prelude::*;

#[derive(Component)]

pub struct Collector {
    pub velocity: f32,
    pub stash_pos: Transform,
    pub returning: bool,
    pub carrying: Option<f32>,
}

#[derive(Event)]
pub struct CollectorSpawnEvent {
    pub spawn_pos: Transform,
}
