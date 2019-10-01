use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: u8,
    y: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PieceState {
    shape: u8,
    pivot: Point,
    rotation: u8,
}
