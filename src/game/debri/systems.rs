use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_spatial::SpatialAccess;
use rand::Rng;

use crate::game::components::Collider;

use super::{
    components::{Debri, NNTree, SpawnDebri},
    DEBRI_SIZE,
};

pub fn debri_movement(
    mut debri_query: Query<(&mut Transform, &mut Debri)>,
    time: Res<Time>,
    treeaccess: Res<NNTree>,
) {
    let mut rng = rand::thread_rng(); // Create a random number generator

    for (mut transform, mut debri) in debri_query.iter_mut() {
        debri.time_alive += time.delta_seconds();

        // If the velocity is too small, skip this debris
        if debri.velocity.length() < 1.0 {
            continue;
        }

        // Delay the start of deceleration
        if debri.time_alive > 1.0 {
            // Increased from 0.5 to 1.0 seconds
            debri.start_deceleration = true;
        }

        if debri.start_deceleration {
            let deceleration_rate = rng.gen_range(50.0..800.0); // Reduced max range
            let velocity_copy = debri.velocity.clone();
            debri.velocity -= velocity_copy.normalize() * deceleration_rate * time.delta_seconds();
        }

        // Apply less frequent random direction change
        if rng.gen_bool(0.1) {
            // 10% chance each frame to change direction
            let angle: f32 = rng.gen_range(-10.0f32..10.0f32).to_radians(); // Reduced angle variation
            let new_velocity = rotate_vector(debri.velocity, angle);
            debri.velocity = new_velocity;
        }

        if debri.velocity.x != 0.0 || debri.velocity.y != 0.0 {
            transform.translation.x += debri.velocity.x * time.delta_seconds();
            transform.translation.y += debri.velocity.y * time.delta_seconds();
        }

        // get x and y from transform in a vector
        let mut debri_pos = Vec2::new(transform.translation.x, transform.translation.y);

        // push nearby debri away
        if let Some(nearest) = treeaccess.nearest_neighbour(debri_pos) {
            let towards = nearest.0 - debri_pos;
            debri_pos -= towards.normalize() * time.delta_seconds() * 64.0;
            transform.translation.x = debri_pos.x;
            transform.translation.y = debri_pos.y;
        }
    }
}
// Function to rotate a vector by a given angle
fn rotate_vector(vec: Vec2, angle: f32) -> Vec2 {
    let (sin_angle, cos_angle) = angle.sin_cos();
    Vec2::new(
        cos_angle * vec.x - sin_angle * vec.y,
        sin_angle * vec.x + cos_angle * vec.y,
    )
}

pub fn despawn_debri(mut commands: Commands, projectile_query: Query<Entity, With<Debri>>) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_debri(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventReader<SpawnDebri>,
) {
    for event in events.read() {
        println!("spawn debri");
        let mut rng = rand::thread_rng();
        let position = event.position.clone();
        let direction = event.direction;
        // Negate and normalize the direction for debris velocity
        let velocity = -direction.normalize() * rng.gen_range(100.0..200.0);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(DEBRI_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },
            Debri {
                velocity: Vec2::new(velocity.x, velocity.y),
                time_alive: 0.0,
                start_deceleration: false,
            },
            Collider {
                size: Vec2::new(DEBRI_SIZE, DEBRI_SIZE),
            },
        ));
    }
}
