use bevy::prelude::*;

use crate::components::{
    grid::{
        grid_height_offset::GridHeigthOffset,
        grid_position::{
            FractionalGridPosition, GridCellPosition, GridPosition, GridPositionComponent,
        },
        movement::GridMovementComponent,
    },
    object::creature::{
        Creature,
        name::CreatureName,
        stats::{CreatureStats, modifiers::definitions::StatModifier},
    },
};

use super::TestingComponent;

pub fn spawn_testing_colonist(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        TestingComponent,
        Creature,
        CreatureStats::new(&[StatModifier::Human]),
        CreatureName {
            name: "Testing creature".to_string(),
        },
        GridPositionComponent::new(GridPosition::new(0, 0)),
        GridMovementComponent::new_with_fractional(
            1000.0,
            GridCellPosition::new(GridPosition::new(20, 20)),
        ),
        GridHeigthOffset::new(Some(1.0)),
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));
}
