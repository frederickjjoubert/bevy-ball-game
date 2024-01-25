use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::collector::components::CollectorSpawnEvent;
use crate::game::ui::spawn_toolbar::components::*;
use crate::game::ui::spawn_toolbar::styles::HOVERED_BUTTON;
use crate::game::ui::spawn_toolbar::styles::NORMAL_BUTTON;
use crate::game::ui::spawn_toolbar::styles::PRESSED_BUTTON;
use crate::game::ui::spawn_toolbar::styles::*;
use crate::game::SimulationState;
use crate::AppState;

pub fn interact_with_button(
    mut events_spawn_collector: EventWriter<CollectorSpawnEvent>,
    // mut events_spawn_shooter: EventWriter<SpawnShooter>,
    // mut events_spawn_building: EventWriter<SpawnBuilding>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &DefaultButton),
        (Changed<Interaction>, With<DefaultButton>),
    >,
) {
    for (interaction, mut color, default_button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                // check the enum type of default button
                match default_button {
                    DefaultButton::Collector => {
                        events_spawn_collector.send(CollectorSpawnEvent {
                            spawn_pos: Transform::from_xyz(100.0, 100.0, 0.0),
                        });
                    }
                    DefaultButton::Shooter => {
                        // events_spawn_shooter.send(SpawnShooter);
                        println!("Shooter");
                    }
                    DefaultButton::Building(building_type) => match building_type {
                        BuldingType::Collector => {
                            println!("Collector");
                        }
                        BuldingType::Shooter => {
                            println!("Shooter");
                        }
                    },
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
