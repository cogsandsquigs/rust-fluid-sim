mod density;
mod particle;

use bevy::prelude::*;
use particle::Particle;

pub const METER_LENGTH: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

/// Setup for the simulation.
fn setup(mut commands: Commands) {
    // Start the camera.
    commands.spawn(Camera2dBundle::default());

    // Spawn a particle.
    let particle = Particle::default();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::WHITE,

                custom_size: Some(Vec2::new(
                    particle.radius * 2. * METER_LENGTH,
                    particle.radius * 2. * METER_LENGTH,
                )),
                ..Default::default()
            },
            ..Default::default()
        },
        particle,
    ));
}
