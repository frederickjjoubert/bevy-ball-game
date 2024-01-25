use bevy::prelude::Component;

pub enum BuldingType {
    Collector,
    Shooter,
}

#[derive(Component)]
pub enum DefaultButton {
    Collector,
    Shooter,
    Building(BuldingType),
}

#[derive(Component)]
pub struct SpawnToolbar;
