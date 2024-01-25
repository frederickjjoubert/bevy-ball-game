use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_spatial::SpatialAccess;

use crate::game::{
    debri::components::{Debri, NNTree},
    score::resources::Score,
};

use super::{
    components::{Collector, CollectorSpawnEvent},
    COLLECTOR_SIZE,
};

pub fn collector_movement(
    mut query: Query<(&mut Transform, &Collector)>,
    time: Res<Time>,
    treeaccess: Res<NNTree>,
) {
    for (mut transform, collector) in query.iter_mut() {
        let collector_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let target_pos = if collector.returning {
            Vec2::new(
                collector.stash_pos.translation.x,
                collector.stash_pos.translation.y,
            )
        } else if let Some(nearest) = treeaccess.nearest_neighbour(collector_pos) {
            nearest.0
        } else {
            collector_pos
        };

        let towards = (target_pos - collector_pos).normalize();
        transform.translation.x += towards.x * time.delta_seconds() * collector.velocity;
        transform.translation.y += towards.y * time.delta_seconds() * collector.velocity;
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
                velocity: 100.0,
                stash_pos: event.spawn_pos,
                returning: false,
                carrying: None,
            },
        ));
    }
}

pub fn check_colision_collector(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &Collector)>,
    mut query_debri: Query<(Entity, &Transform), With<Debri>>,
    mut score: ResMut<Score>,
) {
    for (entity, transform, collector) in query.iter_mut() {
        if collector.returning {
            let distance = transform
                .translation
                .distance(collector.stash_pos.translation);
            if distance < COLLECTOR_SIZE {
                commands.entity(entity).insert(Collector {
                    velocity: collector.velocity,
                    stash_pos: collector.stash_pos,
                    returning: false,
                    carrying: None,
                });

                score.value += 1;
            }
        } else {
            for (entity_debri, transform_debri) in query_debri.iter_mut() {
                let distance = transform.translation.distance(transform_debri.translation);
                if distance < COLLECTOR_SIZE {
                    commands.entity(entity_debri).despawn();

                    commands.entity(entity).insert(Collector {
                        velocity: collector.velocity,
                        stash_pos: collector.stash_pos,
                        returning: true,
                        carrying: Some(1.0),
                    });
                }
            }
        }
    }
}
