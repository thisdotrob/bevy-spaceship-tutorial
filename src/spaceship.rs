use bevy::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_TURN_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let mut transform = Transform::from_translation(STARTING_TRANSLATION);

    transform.rotate_x(1.5708);
    transform.rotate_y(1.5708);

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform,
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyW) {
        velocity.value = -transform.forward() * SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        velocity.value = transform.forward() * SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        transform.rotate_y(time.delta_seconds() * SPACESHIP_TURN_SPEED);
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        transform.rotate_y(time.delta_seconds() * -SPACESHIP_TURN_SPEED);
    }
}
