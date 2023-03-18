use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Debug, Default)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
