use bevy::prelude::*;

use crate::systems::{
    camera::{manual_camera_move, setup_camera},
    grid::position::set_grid_transforms,
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, manual_camera_move.before(set_grid_transforms));
    }
}
