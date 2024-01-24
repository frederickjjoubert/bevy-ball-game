use bevy::prelude::*;
#[derive(Component)]

pub struct Debri {
    pub velocity: Vec2,
    pub time_alive: f32,
    pub start_deceleration: bool,
}

impl Debri {
    // Initialize with a velocity and set time_alive to 0
    pub fn new(velocity: Vec2) -> Self {
        Debri {
            velocity,
            time_alive: 0.0,
            start_deceleration: false,
        }
    }
}
