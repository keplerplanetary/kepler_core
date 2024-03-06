use maths_rs::Vec2d;

use crate::{
    force::gravity_force_vector,
    types::{Body, MoverImplementation, SimulationConfig, System},
};

/// This function maps a system state at time t to a system state at time t + ∆t
pub fn system_timestep(system: System, timestep: f64, config: SimulationConfig) -> System {
    // we could just initialize a new system here, but then we'd have to copy future fields manually
    let mut new_system = system.clone();
    new_system.bodies = vec![];

    // for each body, calculate resulting force
    // system [a, b, c]
    // for a: b-a, c-a
    // for b: b-a, c-b
    // for c: c-a ,c-b
    for body in &system.bodies {
        let mut res_force = Vec2d::new(0.0, 0.0);
        for other in &system.bodies {
            res_force += gravity_force_vector(body, other);
        }

        let mut new_body = body.clone();

        match config.mover_implementation {
            MoverImplementation::EulerExplicit() => {
                mover_euler_explicit(&mut new_body, res_force, timestep)
            }
            MoverImplementation::EulerImplicit() => {
                mover_euler_implicit(&mut new_body, res_force, timestep)
            }
        };

        new_system.bodies.push(new_body);
    }
    new_system
}

///Moves a body with respect to the explicit Euler method
/// in an explicit moving, the new position and velocities only depend on the old values
fn mover_euler_explicit(body: &mut Body, force: Vec2d, timestep: f64) {
    // calculate new position based on old position and velocity
    // pos_new = pos + v * dt
    body.position += body.velocity * timestep;
    // calculate new velocity based on old velocity and force
    // v_new = v + F/m * dt
    body.velocity += force / body.mass * timestep;
}

///Moves a body with respect to the implicit Euler method
/// in an implicit moving, the new position and velocities can depend on the old and new values
fn mover_euler_implicit(body: &mut Body, force: Vec2d, timestep: f64) {
    // calculate new velocity based on old velocity and force
    // v_new = v + F/m * dt
    body.velocity += force / body.mass * timestep;
    // calculate new position from old position and new velocity
    // pos_new = pos + v * dt
    body.position += body.velocity * timestep;
}

#[cfg(test)]
mod tests {

    use crate::{
        constants::{MASS_EARTH, MASS_MOON},
        types::Body,
    };

    use super::*;
    use maths_rs::Vec2d;

    #[test]
    fn test_system_timestep() {
        let mut system = System {
            bodies: vec![
                Body {
                    name: "Earth".to_owned(),
                    mass: MASS_EARTH,
                    position: Vec2d::new(0.0, 0.0),
                    velocity: Vec2d::new(0.0, 0.0),
                },
                Body {
                    name: "Moon".to_owned(),
                    mass: MASS_MOON,
                    position: Vec2d::new(4.0e9, 0.0),
                    velocity: Vec2d::new(0.0, -5.0e3),
                },
            ],
        };

        let timestep = 3600.0;
        for _ in 0..100 {
            system = system_timestep(
                system,
                timestep,
                SimulationConfig {
                    mover_implementation: MoverImplementation::EulerImplicit(),
                },
            );
        }
    }
}
