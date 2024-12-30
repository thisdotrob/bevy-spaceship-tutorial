mod camera;
mod debug;
mod movement;
mod markers;
mod spaceship;
mod asteroids;
mod asset_loader;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use markers::MarkerPlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.0, 0.3)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MarkerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .run();
}
