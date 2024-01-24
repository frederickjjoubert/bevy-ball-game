use super::components::Collider;
use crate::game::SimulationState;
use bevy::{prelude::*, sprite::collide_aabb::*};

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.get() == &SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if simulation_state.get() == &SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
            println!("Simulation Running.");
        }
    }
}

pub fn check_collisions(
    mut commands: Commands,
    collider_query: Query<(Entity, &Transform, &Collider)>,
) {
    let colliders: Vec<(Entity, Transform, Collider)> = collider_query
        .iter()
        .map(|(entity, transform, collider)| (entity, *transform, *collider))
        .collect();

    for (entity, transform, collider) in &colliders {
        for (other_entity, other_transform, other_collider) in &colliders {
            if entity != other_entity {
                let collision = collide(
                    transform.translation,
                    collider.size,
                    other_transform.translation,
                    other_collider.size,
                );

                let mut reflect_x = false;
                let mut reflect_y = false;

                if let Some(collision) = collision {
                    if collision == Collision::Left || collision == Collision::Right {
                        reflect_x = true;
                    }
                    if collision == Collision::Top || collision == Collision::Bottom {
                        reflect_y = true;
                    }
                }

                // move the collider some pixels away from the other collider
                if reflect_x {
                    let x = if collision.unwrap() == Collision::Left {
                        other_transform.translation.x
                            - collider.size.x
                            - other_collider.size.x
                    } else {
                        other_transform.translation.x
                            + collider.size.x
                            + other_collider.size.x
                    };
                    commands.entity(*entity).insert(Transform::from_xyz(
                        x,
                        transform.translation.y,
                        0.0,
                    ));
                }

                if reflect_y {
                    let y = if collision.unwrap() == Collision::Bottom {
                        other_transform.translation.y
                            - collider.size.y / 2.0
                            - other_collider.size.y / 2.0
                    } else {
                        other_transform.translation.y
                            + collider.size.y / 2.0
                            + other_collider.size.y / 2.0
                    };
                    commands.entity(*entity).insert(Transform::from_xyz(
                        transform.translation.x,
                        y,
                        0.0,
                    ));
                }
            }
        }
    }
}
