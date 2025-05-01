mod smooth;
mod traits;
mod translate;

pub use smooth::*;
pub use traits::*;
pub use translate::*;

use std::f32;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct SculptParams {
    max_move_dist_squared: f32,
    min_edge_length_squared: f32,
    max_edge_length_squared: f32,
}

impl SculptParams {
    pub fn new(max_edge_length: f32) -> Self {
        let max_edge_length_squared = max_edge_length * max_edge_length;

        Self {
            max_move_dist_squared: max_edge_length_squared * 0.11,
            min_edge_length_squared: max_edge_length_squared * 0.24,
            max_edge_length_squared,
        }
    }
}
