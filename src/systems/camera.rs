use bevy::prelude::*;

use crate::components::{
    camera::CameraComponent,
    grid::{
        GRID_SIZE,
        grid_height_offset::GridHeigthOffset,
        grid_position::{GridPosition, GridPositionComponent},
    },
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        CameraComponent { camera_speed: 10.0 },
        Camera3d::default(),
        // 1) Posicionar la cámara arriba
        // 2) Apuntar el forward local (−Z) hacia abajo (−Y)
        // 3) Definir up local (+Y) como +Z del mundo
        Transform::from_xyz(0.0, 18.0, 0.0)
            .looking_at(Vec3::ZERO, -Vec3::X)
            .with_rotation(Quat::from_xyzw(
                -f32::sqrt(2.0) / 2.0, // x
                0.0,                   // y
                0.0,                   // z
                f32::sqrt(2.0) / 2.0,  // w
            )),
        GridPositionComponent::new(GridPosition::new(0, 0)),
        GridHeigthOffset::new(Some(18.0)),
    ));
}

pub fn manual_camera_move(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut CameraComponent,
            &mut GridPositionComponent,
            &GridHeigthOffset,
            &mut Transform,
        ),
        With<CameraComponent>,
    >,
) {
    if let Ok((camera, mut grid_pos_component, grid_height_offset, mut transform)) =
        query.single_mut()
    {
        let mut dir = Vec2::ZERO;
        let delta_s = time.delta_secs();

        if input.pressed(KeyCode::KeyW) {
            dir.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            dir.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            dir.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            dir.x += 1.0;
        }

        if dir != Vec2::ZERO {
            dir = dir.normalize();
            // Movemos la cámara usando el método encapsulado
            grid_pos_component
                .cell_pos
                .move_in_grid(dir, camera.camera_speed, delta_s);
        }
    }
}
