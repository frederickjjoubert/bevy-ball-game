use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub struct Collider {
    pub size: Vec2,
}
#[derive(Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
