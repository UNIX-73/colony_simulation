use bevy::prelude::*;

use crate::components::{
    grid::{
        grid_fractional_position::GridFractionalPosition, grid_height_offset::GridHeigthOffset,
        grid_position::GridPosition,
    },
    object::creature::{Creature, name::CreatureName},
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
        CreatureName {
            name: "Testing creature".to_string(),
        },
        GridPosition { x: 0, y: 0 },
        GridFractionalPosition::default(),
        GridHeigthOffset::new(Some(1.0)),
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));
}
