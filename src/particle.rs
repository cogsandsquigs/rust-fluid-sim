use bevy::prelude::*;

/// A basic particle, with a position, velocity, acceleration, and mass.
#[derive(Clone, Component)]
pub struct Particle {
    /// The acceleration of the particle, in meters per second squared (m/s^2).
    pub acceleration: Vec2,

    /// The velocity of the particle, in meters per second (m/s).
    pub velocity: Vec2,

    /// The mass of the particle, in kilograms (kg).
    pub mass: f32,

    /// The radius of the particle, in meters (m).
    pub radius: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            acceleration: Vec2::ZERO,
            velocity: Vec2::ZERO,
            mass: 1.0,
            radius: 1.0,
        }
    }
}
