use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}
