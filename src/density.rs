// The smoothing function for the influence of a single particle, ranging from 0 (no influence)
// to 1 (total influence). `radius` is the maximum radius of influence, and `distance` is the
// distance from the particle to the point at which the influence is being calculated.
pub fn smoothing(radius: f64, distance: f64) -> f64 {
    let q = f64::max(0., radius - distance); // Clamp to 0, so we don't get negative values (which don't make sense)

    q * q * q // Cubic smoothing function
}

/// The volume of the smoothing function for a single particle, in 3D. Used when calculating the
/// density of particles, as we need to divide by the volume of the smoothing influence/function
/// to account for the fact that the density is the mass per unit volume.
pub fn smoothing_volume_3d(radius: f64) -> f64 {
    // derived via Wolfram|Alpha by triple-integrating the smoothing function `smoothing` over a sphere
    // with radius `radius`. Don't forget the Jacobian!
    std::f64::consts::PI * radius.powi(6) / 15.
}

/// The volume of the smoothing function for a single particle, in 2D. Used when calculating the
/// density of particles, as we need to divide by the volume of the smoothing influence/function
/// to account for the fact that the density is the mass per unit volume.
pub fn smoothing_volume_2d(radius: f64) -> f64 {
    // derived via Wolfram|Alpha by double-integrating the smoothing function `smoothing` over a circle
    // with radius `radius`. Don't forget the Jacobian!
    std::f64::consts::PI * radius.powi(5) / 10.
}
