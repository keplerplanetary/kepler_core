
/// Calculates the gravitational force between two masses.
///
/// usage
/// ````rust
/// use planety::gravity_force;
/// 
/// let mass_earth = 5.972e24;
/// let mass_sun = 1.989e30;
/// let distance = 149597870700.0;
/// let result = gravity_force(mass_earth, mass_sun, distance);
/// ````
/// F = (G * m1 * m2)/(r*r)
/// G = 6.674×10−11 m3⋅kg−1⋅s−2
pub fn gravity_force(mass_a: f64, mass_b: f64, distance: f64) -> f64 {
    let g: f64 = 6.674e-11;
    g * mass_a * mass_b / (distance.powf(2.0))
}

pub struct System {
    bodies: Vec<Body>,
}



pub struct Body{
    mass: f64,
    name: String,
    position: CartesianPosition,
    velocity: CartesianVelocity,
}

pub struct CartesianPosition{
    x: f64,
    y: f64,
}

pub struct CartesianVelocity{
    vx: f64,
    vy: f64,
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
