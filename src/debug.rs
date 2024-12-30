use bevy::prelude::*;

use crate::spaceship::Spaceship;

const DEBUG_TIME_SECONDS: f32 = 2.0;

#[derive(Resource, Debug)]
pub struct DebugTimer {
    timer: Timer,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugTimer {
            timer: Timer::from_seconds(DEBUG_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, print_position);
    }
}

fn print_position(
    mut query: Query<&mut Transform, With<Spaceship>>,
    mut debug_timer: ResMut<DebugTimer>,
    time: Res<Time>,
) {
    debug_timer.timer.tick(time.delta());

    if !debug_timer.timer.just_finished() {
        return;
    }

    let mut transform = query.single_mut();

    info!("local_x {:?},", transform.local_x());
    info!("local_y {:?},", transform.local_y());
    info!("local_z {:?},", transform.local_z());
    info!("---------------");
}
