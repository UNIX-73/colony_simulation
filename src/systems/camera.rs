use bevy::prelude::*;

use crate::components::{
    camera::CameraComponent,
    grid::{
        grid_fractional_position::GridFractionalPosition, grid_height_offset::GridHeigthOffset,
        grid_position::GridPosition,
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
        GridPosition { x: 0, y: 0 },
        GridFractionalPosition { x: 0.0, y: 0.0 },
        GridHeigthOffset::new(Some(18.0)),
    ));
}

pub fn manual_camera_move(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut CameraComponent,
            &mut GridFractionalPosition,
            &GridHeigthOffset,
            &mut Transform,
        ),
        With<CameraComponent>,
    >,
) {
    if let Ok((camera, mut grid_frac_pos, grid_height_offset, mut transform)) =
        query.get_single_mut()
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
            // Aquí modificamos el sistema de grid fraccional, no el transform directamente
            grid_frac_pos.x += dir.x * delta_s * camera.camera_speed;
            grid_frac_pos.y += dir.y * delta_s * camera.camera_speed;
        }

        if input.pressed(KeyCode::ArrowRight) {
            transform.rotate_y(-45f32.to_radians() * delta_s);
        }
        if input.pressed(KeyCode::ArrowLeft) {
            transform.rotate_y(45f32.to_radians() * delta_s);
        }

        // Actualizamos el Transform.position a partir de GridFractionalPosition
        transform.translation.x = grid_frac_pos.x;
        transform.translation.z = grid_frac_pos.y;
        transform.translation.y = grid_height_offset.get_offset();

        println!(
            "Cam pos (grid): ({}, {})  ({})",
            grid_frac_pos.x, grid_frac_pos.y, transform.rotation
        );
    }
}
