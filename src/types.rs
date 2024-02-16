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
