use bevy::prelude::*;

use crate::components::scene::SceneComponent;

pub fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SceneComponent,
        Mesh3d(meshes.add(Cuboid::new(10000.0, 10.0, 10000.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(156, 107, 3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn setup_lights(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            color: Color::srgb_u8(200, 215, 230),
            illuminance: 500.0,
            shadows_enabled: false,
            shadow_depth_bias: 0.0,
            shadow_normal_bias: 0.0,
        },
        Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Y),
    ));
}
