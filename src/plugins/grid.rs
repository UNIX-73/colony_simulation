use bevy::prelude::*;

use crate::systems::grid::{movement::grid_movement, position::set_grid_transforms};

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grid_movement)
            .add_systems(PostUpdate, set_grid_transforms);
    }
}
