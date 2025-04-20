mod components;
mod events;
mod plugins;
mod resources;
mod systems;
mod utils;

use bevy::prelude::*;
use plugins::{
    camera::CameraPlugin, creature::stats::CreatureStatsPlugin, grid::GridPlugin,
    scene::ScenePlugin,
};
use utils::test::TestingPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        ScenePlugin,
        CameraPlugin,
        GridPlugin,
        TestingPlugin,
        CreatureStatsPlugin,
    ));

    app.run();
}
