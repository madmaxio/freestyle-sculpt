use glam::Vec3;

use super::DistanceCalculator;

#[derive(Debug, Clone, Copy)]
pub struct L2;

impl DistanceCalculator for L2 {
    fn distance_squared(&self, a: Vec3, b: Vec3) -> f32 {
        a.distance_squared(b)
    }
}
