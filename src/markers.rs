use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Component, Debug)]
pub struct Marker;

pub struct MarkerPlugin;

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_markers);
    }
}

fn spawn_markers(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    for n in [10., 20., 30., 40., 50.] {
        // X axis markers
        let translation = Vec3::new(n, 0., 0.);
        commands.spawn((
            SceneBundle {
                scene: scene_assets.astronaut_frog.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
            Marker,
        ));

        // Y axis markers
        let translation = Vec3::new(0., n, 0.);
        commands.spawn((
            SceneBundle {
                scene: scene_assets.astronaut_mouse.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
            Marker,
        ));

        // Z axis markers
        let translation = Vec3::new(0., 0., n);
        commands.spawn((
            SceneBundle {
                scene: scene_assets.astronaut_bird.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
            Marker,
        ));
    }
}
