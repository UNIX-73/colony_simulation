pub mod spawn_colonist;

use bevy::prelude::*;
use spawn_colonist::spawn_testing_colonist;

#[derive(Component)]
pub struct TestingComponent;

pub struct TestingPlugin;
impl Plugin for TestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_testing_colonist);
    }
}
