use bevy::prelude::{Resource, Timer, TimerMode};

pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        return StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}
