mod components;
mod events;
mod plugins;
mod resources;
mod systems;
mod utils;

use bevy::prelude::*;
use plugins::{camera::CameraPlugin, grid::GridPlugin, scene::ScenePlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ScenePlugin, CameraPlugin, GridPlugin));

    app.run();
}
