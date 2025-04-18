use bevy::prelude::*;

use crate::systems::scene::{setup_ground, setup_lights};

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_lights, setup_ground));
    }
}
