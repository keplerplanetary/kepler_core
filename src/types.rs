use maths_rs::Vec2d;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct System {
    pub bodies: Vec<Body>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub id: String,
    pub name: String,
    pub color: Color,
    pub diameter: f64, // m
    pub mass: f64,     // kg
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
