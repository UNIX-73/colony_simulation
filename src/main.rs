mod components;
mod events;
mod plugins;
mod resources;
mod systems;
mod utils;

use bevy::{prelude::*};
use plugins::{
    camera::CameraPlugin, chunks::ChunksPlugin, creature::stats::CreatureStatsPlugin,
    grid::GridPlugin, render::RenderPlugin, scene::ScenePlugin,
};
use utils::test::TestingPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        ChunksPlugin,
        ScenePlugin,
        CameraPlugin,
        GridPlugin,
        TestingPlugin,
        CreatureStatsPlugin,
        RenderPlugin,
    ));

    app.run();
}
