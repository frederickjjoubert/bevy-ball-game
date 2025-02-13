use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub direction: Vec2,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            direction: Vec2::new(x, y).normalize(),
        }
    }
}
