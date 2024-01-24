use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::Debri;

pub fn debri_movement(mut debri_query: Query<(&mut Transform, &mut Debri)>, time: Res<Time>) {
    let mut rng = rand::thread_rng(); // Create a random number generator

    for (mut transform, mut debri) in debri_query.iter_mut() {
        debri.time_alive += time.delta_seconds();

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
