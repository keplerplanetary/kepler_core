use maths_rs::{length, Vec2d};

/// Calculates the gravitational force between two masses.
///
/// usage
/// ```rust
/// use planety_core::gravity_force;
///
/// let mass_earth = 5.972e24;
/// let mass_sun = 1.989e30;
/// let distance = 149597870700.0;
/// let result = gravity_force(mass_earth, mass_sun, distance);
/// ```
/// F = (G * m1 * m2)/(r*r)
/// G = 6.674×10−11 m3⋅kg−1⋅s−2
pub fn gravity_force(mass_a: f64, mass_b: f64, distance: f64) -> f64 {
    let g: f64 = 6.674e-11;
    g * mass_a * mass_b / (distance.powf(2.0))
}

pub fn gravity_force_vector(body_a: Body, body_b: Body) -> Vec2d {
    let mass_a = body_a.mass;
    let mass_b = body_b.mass;
    let distance_vec = body_a.position - body_b.position;
    let distance = length(body_a.position);
    Vec2d::new(0.0, 0.0)
}

pub struct System {
    bodies: Vec<Body>,
}

pub struct Body {
    mass: f64,
    name: String,
    position: Vec2d,
    velocity: Vec2d,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn force_between_earth_and_sun() {
        let d = 149597870700.0;

        let mass_earth = 5.972e24;
        let mass_sun = 1.989e30;

        let result = gravity_force(mass_earth, mass_sun, d);
        assert!(3.5423377e22 > result);
        assert!(3.5423376e22 < result);
    }
}
