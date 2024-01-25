use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_spatial::SpatialAccess;

use crate::game::debri::components::NNTree;

use super::{components::Building, BUILDING_SIZE};

pub fn building_movement(
    mut query: Query<&mut Transform, With<Building>>,
    time: Res<Time>,
    treeaccess: Res<NNTree>,
) {
    for mut transform in query.iter_mut() {
        let building_pos = Vec2::new(transform.translation.x, transform.translation.y);
        // move to
        if let Some(nearest) = treeaccess.nearest_neighbour(building_pos) {
            let towards = nearest.0 - building_pos;
            transform.translation.x += towards.x * time.delta_seconds() * 64.0;
            transform.translation.y += towards.y * time.delta_seconds() * 64.0;
        }
    }
}

pub fn despawn_building(mut commands: Commands, query: Query<Entity, With<Building>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_building(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Cube {
                        size: BUILDING_SIZE,
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Building,
    ))
}
