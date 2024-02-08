use maths_rs::{dot, length};

use crate::{
    constants::G,
    types::{Body, System},
};

// Calculate the total energy of the system. The total energy is composed of potential energy and kinetic energy and it should be constant over time
pub fn calculate_system_energy(system: System) -> f64 {
    let sum_of_kinetic: f64 = system
        .bodies
        .iter()
        .map(|s| calculate_kinetic_energy(s))
        .sum();

    let mut looped_bodies: Vec<&Body> = Vec::new();

    let sum_of_potential = system
        .bodies
        .iter()
        .map(|body| {
            let potential_energy = looped_bodies
                .iter()
                .map(|other| calculate_potential_energy(body, other))
                .sum::<f64>();
            looped_bodies.push(body);
            potential_energy
        })
        .sum::<f64>();

    sum_of_kinetic + sum_of_potential
}

/// Calculate the kinetic energy of a body
///
/// the kinetic energy is defined as `E = 0.5 * m * v^2`
pub fn calculate_kinetic_energy(body: &Body) -> f64 {
    0.5 * body.mass * dot(body.velocity, body.velocity)
}

/// calculates the potential energy of receiving_body in the gravitational field of exerting_body
pub fn calculate_potential_energy(receiving_body: &Body, exerting_body: &Body) -> f64 {
    if receiving_body == exerting_body {
        // the force between two identical bodies is 0
        // if we try to calculate it anyways, it will be f64::Nan.
        0.0
    } else {
        let mass_a = receiving_body.mass;
        let mass_b = exerting_body.mass;
        let distance_vec = receiving_body.position - exerting_body.position;
        let distance = length(distance_vec).abs();

        (G * mass_a * mass_b / distance) * -1.0
    }
}
