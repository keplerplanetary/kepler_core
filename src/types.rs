use maths_rs::Vec2d;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SimulationConfig {
    pub mover_implementation: MoverImplementation,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            mover_implementation: MoverImplementation::EulerExplicit(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum MoverImplementation {
    EulerExplicit(),
    EulerImplicit(),
}
