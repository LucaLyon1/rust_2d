mod camera;
mod character;
mod map;
mod movement;

use bevy::prelude::*;
use camera::CameraPlugin;
use character::CharacterPlugin;
use map::MapPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
