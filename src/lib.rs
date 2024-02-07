use maths_rs::{dot, length, Vec2d};
use serde::{Deserialize, Serialize};

/// Calculates the gravitational force between two masses.
///
/// usage
/// ```rust
/// use kepler_core::gravity_force;
///
/// let mass_earth = 5.972e24;
/// let mass_sun = 1.989e30;
/// let distance = 149597870700.0;
/// let result = gravity_force(mass_earth, mass_sun, distance);
/// ```
/// F = (G * m1 * m2)/(r*r)
/// G = 6.674×10−11 m3⋅kg−1⋅s−2
pub fn gravity_force(mass_a: f64, mass_b: f64, distance: f64) -> f64 {
    G * mass_a * mass_b / (distance.powf(2.0))
}

/// Calculates the gravitational force between two masses in vector form.
pub fn gravity_force_vector(receiving_body: &Body, exerting_body: &Body) -> Vec2d {
    if receiving_body == exerting_body {
        // the force between two identical bodies is 0
        // if we try to calculate it anyways, it will be f64::Nan.
        Vec2d::new(0.0, 0.0)
    } else {
        let mass_a = receiving_body.mass;
        let mass_b = exerting_body.mass;
        let distance_vec = receiving_body.position - exerting_body.position;
        let distance = length(distance_vec);
        let unit_vector = distance_vec / distance;
        unit_vector * gravity_force(mass_a, mass_b, distance) * -1.0
    }
}

static G: f64 = 6.674e-11;

pub fn system_timestep(system: System, timestep: f64) -> System {
    let mut new_system = system.clone();
    new_system.bodies = vec![];

    // for each body, calculate resulting force
    // system [a, b, c]
    // for a: b-a, c-a
    // for b: b-a, c-b
    // for c: c-a ,c-b
    for body in system.bodies.clone() {
        let mut res_force = Vec2d::new(0.0, 0.0);
        for other in system.bodies.clone() {
            res_force += gravity_force_vector(&body, &other);
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

pub fn calculate_system_energy(system: System) -> f64 {
    let mut sum_of_kinetic: f64 = 0.0;
    let mut sum_of_potential: f64 = 0.0;
    let mut looped_bodies: Vec<Body> = Vec::new();

    for body in system.bodies {
        sum_of_kinetic += calculate_kinetic_energy(&body);
        for other in looped_bodies.iter() {
            sum_of_potential += calculate_potential_energy(&body, other);
        }
        looped_bodies.push(body.clone());
    }

    sum_of_kinetic + sum_of_potential
}

pub fn calculate_kinetic_energy(body: &Body) -> f64 {
    0.5 * body.mass * dot(body.velocity, body.velocity)
}

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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct System {
    pub bodies: Vec<Body>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub mass: f64,
    pub name: String,
    pub position: Vec2d,
    pub velocity: Vec2d,
}

#[cfg(test)]
mod tests {
    use super::*;
    use maths_rs::Vec2d;

    static MASS_EARTH: f64 = 5.972e24;
    static MASS_MOON: f64 = 7.34e22;
    static MASS_SUN: f64 = 1.989e30;

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

        println!("System start\n {:#?}", &system);

        let timestep = 3600.0;
        for _ in 0..100 {
            system = system_timestep(system, timestep);
        }
        println!("System end\n {:#?}", &system);
    }

    #[test]
    fn force_between_earth_and_sun() {
        let d = 149597870700.0;

        let result = gravity_force(MASS_EARTH, MASS_SUN, d);
        assert!(3.5423377e22 > result);
        assert!(3.5423376e22 < result);
    }

    #[test]
    fn force_between_earth_and_sun_vector() {
        let body_a = Body {
            mass: 5.972e24,
            name: "Earth".to_owned(),
            position: Vec2d::new(1.495978707e11, 0.0),
            velocity: Vec2d::new(0.0, 2.98e7),
        };
        let body_b = Body {
            mass: 1.989e30,
            name: "Sun".to_owned(),
            position: Vec2d::new(0.0, 0.0),
            velocity: Vec2d::new(0.0, 0.0),
        };
        let result = gravity_force_vector(&body_a, &body_b);
        let distance_vec = body_a.position - body_b.position;
        // The resulting force points toward the other body
        assert!(result.x * distance_vec.y == result.y * distance_vec.x);
    }
}
