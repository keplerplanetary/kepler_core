use maths_rs::{length, Vec2d};

use crate::{constants::G, types::Body};

/// Calculates the gravitational force between two masses.
///
/// usage
/// ```rust
/// use kepler_core::force::gravity_force;
/// use kepler_core::constants::{MASS_EARTH, MASS_SUN};
///
/// let distance = 149597870700.0;
/// let result = gravity_force(MASS_EARTH, MASS_SUN, distance);
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

#[cfg(test)]
mod tests {
    use maths_rs::Vec2d;

    use crate::{
        constants::{MASS_EARTH, MASS_SUN},
        force::{gravity_force, gravity_force_vector},
        types::Body,
    };

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
        // The resulting force points towards the other body
        assert!(result.x * distance_vec.y == result.y * distance_vec.x);
    }
}
