use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CameraComponent {
    pub camera_speed: f32,
}
