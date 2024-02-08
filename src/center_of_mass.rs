use maths_rs::Vec2d;

use crate::types::System;

/// Calculates the center of mass of a system
pub fn calculcate_center_of_mass(system: &System) -> Vec2d {
    system
    .bodies
    .iter()
    .map(|b| b.mass * b.position)
    .reduce(|v_sum, v| v_sum + v)
    .expect("Calculation of center of mass should have worked, because we expect that non-empty systems are simulated")
}
