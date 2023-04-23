use bevy::prelude::Resource;
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Debug, Resource, Default)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
