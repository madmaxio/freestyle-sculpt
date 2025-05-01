mod sphere_with_falloff;
mod traits;

pub use sphere_with_falloff::*;
pub use traits::*;

use glam::Vec3;

fn get_sphere_with_falloff_weight_callback(
    input_pos: Vec3,
    radius: f32,
    falloff: f32,
) -> Box<dyn Fn(Vec3) -> f32> {
    Box::new(move |pos: Vec3| {
        let distance = pos.distance(input_pos);
        let rf = radius + falloff;

        if distance <= radius {
            1.0
        } else if distance <= rf {
            (rf - distance) / falloff
        } else {
            0.0
        }
    })
}
