use maths_rs::Vec2d;

use crate::{force::gravity_force_vector, types::System};

/// This function maps a system state at time t to a system state at time t + ∆t
pub fn system_timestep(system: System, timestep: f64) -> System {
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
        // calculate new velocity
        // v_new = v + F/m * dt
        let v_new = body.velocity + res_force / body.mass * timestep;
        // calculate new position
        // pos_new = pos + v * dt
        let pos_new = body.position + v_new * timestep;

        let mut new_body = body.clone();
        new_body.velocity = v_new;
        new_body.position = pos_new;

        new_system.bodies.push(new_body);
    }
    new_system
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
            system = system_timestep(system, timestep);
        }
    }
}
