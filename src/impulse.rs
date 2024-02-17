use maths_rs::Vec2d;

use crate::types::{Body, System};

/// calculates the impulse of a single body
pub fn calculate_impulse(body: &Body) -> Vec2d {
    body.mass * body.velocity
}

/// calculates the impulse of the whole system, which should be constant over time
pub fn calculate_total_impulse(system: &System) -> Vec2d {
    system
        .bodies
        .iter()
        .map(calculate_impulse)
        .reduce(|vec_sum, v| vec_sum + v)
        .expect("No empty systems")
}

/*

// The computation of the angular momentum only makes sense in 3d

/// Calculates the angular momentum of a single body
pub fn calculate_angular_momentum(body: &Body) -> Vec2d {
    body.position
}

/// Calculates the total angular momentum of the system, which should be constant over time
pub fn calculate_total_angular_momentum(system: &System) -> Vec2d {
    system
        .bodies
        .iter()
        .map(|b| calculate_angular_momentum(b))
        .reduce(|vec_sum, v| vec_sum + v)
        .expect("No empty systems")
}

 */
