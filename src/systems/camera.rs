use bevy::prelude::*;

use crate::components::camera::CameraComponent;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        CameraComponent { camera_speed: 10.0 },
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn manual_camera_move(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut CameraComponent, &mut Transform), With<CameraComponent>>,
) {
    if let Ok((camera, mut transform)) = query.get_single_mut() {
        let mut dir = Vec2::ZERO;

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
            transform.translation.x += dir.x * time.delta_secs() * camera.camera_speed;
            transform.translation.z += dir.y * time.delta_secs() * camera.camera_speed;
        }
    }
}
