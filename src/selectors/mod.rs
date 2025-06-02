mod distance;
mod metric_with_falloff;
mod surface_metric_with_falloff;
mod traits;

pub use distance::*;
pub use metric_with_falloff::*;
pub use surface_metric_with_falloff::*;
pub use traits::*;

use glam::Vec3;
use hashbrown::HashSet;
use mesh_graph::{FaceId, MeshGraph, VertexId};

pub type FalloffFn = fn(f32) -> f32;

pub const LINEAR_FALLOFF: FalloffFn = |x| x;

pub const SMOOTH_FALLOFF: FalloffFn = |x| {
    let x2 = x * x;
    3.0 * x2 - 2.0 * x2 * x
};

fn get_sphere_with_falloff_weight_callback<D: DistanceCalculator + Copy + 'static>(
    input_pos: Vec3,
    radius: f32,
    falloff: f32,
    falloff_func: FalloffFn,
    distance_calculator: D,
) -> Box<dyn Fn(Vec3) -> f32> {
    Box::new(move |pos: Vec3| {
        let distance = distance_calculator.distance_squared(input_pos, pos).sqrt();
        let rf = radius + falloff;

        if distance <= radius {
            1.0
        } else if distance <= rf {
            falloff_func((rf - distance) / falloff)
        } else {
            0.0
        }
    })
}

fn faces_incident_to_vertices(
    vertices: impl IntoIterator<Item = VertexId>,
    mesh_graph: &MeshGraph,
) -> HashSet<FaceId> {
    let mut faces = HashSet::new();

    for vertex_id in vertices {
        for face in mesh_graph.vertices[vertex_id].faces(mesh_graph) {
            faces.insert(face);
        }
    }

    faces
}
