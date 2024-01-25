use bevy::prelude::*;

pub const PROJECTILE_SPAWN_TIME: f32 = 0.70;

#[derive(Resource)]
pub struct ProjectileSpawnTimer {
    pub timer: Timer,
    pub spawn_multiplier: f32,
}

impl Default for ProjectileSpawnTimer {
    fn default() -> ProjectileSpawnTimer {
        ProjectileSpawnTimer {
            timer: Timer::from_seconds(PROJECTILE_SPAWN_TIME, TimerMode::Repeating),
            spawn_multiplier: 1.0,
        }
    }
}
