use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_spatial::SpatialAccess;

use crate::game::debri::components::NNTree;

use super::{
    components::{Collector, CollectorSpawnEvent},
    COLLECTOR_SIZE,
};

pub fn collector_movement(
    mut query: Query<&mut Transform, With<Collector>>,
    time: Res<Time>,
    treeaccess: Res<NNTree>,
) {
    for mut transform in query.iter_mut() {
        let collector_pos = Vec2::new(transform.translation.x, transform.translation.y);
        // move to
        if let Some(nearest) = treeaccess.nearest_neighbour(collector_pos) {
            let towards = nearest.0 - collector_pos;
            transform.translation.x += towards.x * time.delta_seconds() * 64.0;
            transform.translation.y += towards.y * time.delta_seconds() * 64.0;
        }
    }
}

pub fn despawn_collector(mut commands: Commands, query: Query<Entity, With<Collector>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_collector(
    mut events: EventReader<CollectorSpawnEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in events.read() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(COLLECTOR_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_xyz(
                    event.spawn_pos.translation.x,
                    event.spawn_pos.translation.y,
                    0.0,
                ),
                ..Default::default()
            },
            Collector {
                velocity: Vec2::new(500.0, 0.0),
            },
        ));
    }
}
