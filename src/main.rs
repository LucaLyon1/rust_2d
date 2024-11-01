mod camera;
mod map;

use bevy::prelude::*;
use camera::CameraPlugin;
use map::MapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
