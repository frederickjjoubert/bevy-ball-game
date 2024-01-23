use bevy::prelude::*;

pub struct GameOver {
    pub score: u32,
}

impl Event for GameOver {}
