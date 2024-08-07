use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}
