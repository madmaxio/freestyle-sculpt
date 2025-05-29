mod sphere_with_falloff;
mod surface_sphere_with_falloff;
mod traits;

use hashbrown::HashSet;
use mesh_graph::{FaceId, MeshGraph, VertexId};
pub use sphere_with_falloff::*;
pub use surface_sphere_with_falloff::*;
pub use traits::*;

use glam::Vec3;

pub const LINEAR_FALLOFF: fn(f32) -> f32 = |x| x;

pub const SMOOTH_FALLOFF: fn(f32) -> f32 = |x| {
    let x2 = x * x;
    3.0 * x2 - 2.0 * x2 * x
};

fn get_sphere_with_falloff_weight_callback(
    input_pos: Vec3,
    radius: f32,
    falloff: f32,
    falloff_func: fn(f32) -> f32,
) -> Box<dyn Fn(Vec3) -> f32> {
    Box::new(move |pos: Vec3| {
        let distance = pos.distance(input_pos);
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
