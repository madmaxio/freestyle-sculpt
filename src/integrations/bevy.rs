use bevy::prelude::*;

use crate::ray::Ray;

impl From<Ray3d> for Ray {
    fn from(ray: Ray3d) -> Self {
        Self {
            origin: ray.origin,
            direction: ray.direction.into(),
        }
    }
}
