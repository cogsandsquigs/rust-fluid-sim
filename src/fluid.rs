/// The `Fluid` struct represents a single fluid, with a given density, viscosity, and smoothing
/// radius. It also contains a list of particles that are part of the fluid.
#[derive(Debug, Clone)]
pub struct Fluid {
    /// Smoothing radius, in meters (m)
    pub smoothing_radius: f64,

    /// The mass of an individual particle, in kilograms (kg)
    pub particle_mass: f64,
}

/// Fluids have a collection of `Particle`s that are part of them. Each particle has a position,
/// velocity, and density. Right now, this is defining the `Particle` struct as an entity.
#[derive(Debug, Clone, Copy)]
pub struct Particle(u64);

// /// Every fluid has a collection of `Particle`s that are part of it. Each
