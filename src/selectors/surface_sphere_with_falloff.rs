use glam::Vec3;

use crate::meshgraph::{BiggerLoop, Face, MeshGraph, Selection, SelectionOps};

use super::{get_sphere_with_falloff_weight_callback, MeshSelector, WeightedSelection};

/// Generates a selection on the surface of a mesh that is within a sphere with a falloff and that
/// is limited to be connected to the input face.
pub struct SurfaceSphereWithFalloff {
    /// The radius of the sphere.
    radius: f32,

    /// The falloff distance of the sphere. This means that the influence
    /// decreases from the radius to the radius + falloff.
    /// The way the influence decreases is controlled by `falloff_func`.
    falloff: f32,

    /// The falloff function used to calculate the weight of the selection.
    /// It receives values from 0.0 to 1.0 and has to return a value in the same range.
    /// Simply returning the input value is a linear falloff.
    falloff_func: fn(f32) -> f32,
}

impl SurfaceSphereWithFalloff {
    #[inline]
    pub fn new(radius: f32, falloff: f32, falloff_func: fn(f32) -> f32) -> Self {
        Self {
            radius,
            falloff,
            falloff_func,
        }
    }
}

impl MeshSelector for SurfaceSphereWithFalloff {
    fn select(
        &self,
        mesh_graph: &MeshGraph,
        input_pos: Vec3,
        input_face: Face,
    ) -> WeightedSelection {
        let sum = self.radius + self.falloff;
        let max_dist_sqr = sum * sum;

        let mut selection = Selection::default();

        let one_ring = mesh_graph.vertices[mesh_graph.halfedges[input_face.halfedge].end_vertex]
            .one_ring(mesh_graph);

        let mut potential_loop = BiggerLoop {
            visited_vertices: one_ring
                .iter()
                .map(|he_id| mesh_graph.halfedges[*he_id].end_vertex)
                .collect(),
            grown_loop: one_ring,
        };

        selection.insert(input_face.id);

        loop {
            let mut all_outside = true;

            for v_id in potential_loop.visited_vertices.iter().copied() {
                let pos = mesh_graph.positions[v_id];

                if pos.distance_squared(input_pos) <= max_dist_sqr {
                    selection.insert(v_id);
                    all_outside = false;
                }
            }

            if all_outside {
                break;
            }

            potential_loop = potential_loop.grown_loop.bigger_loop(mesh_graph);
        }

        WeightedSelection {
            selection,
            get_weight: get_sphere_with_falloff_weight_callback(
                input_pos,
                self.radius,
                self.falloff,
                self.falloff_func,
            ),
        }
    }
}
