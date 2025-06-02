use glam::Vec3;
use hashbrown::HashSet;

use mesh_graph::{Face, MeshGraph, Selection};

use super::{
    DistanceCalculator, FalloffFn, L2, MeshSelector, WeightedSelection, faces_incident_to_vertices,
    get_sphere_with_falloff_weight_callback,
};

/// Generates a selection on the surface of a mesh that is within a sphere with a falloff and that
/// is limited to be connected to the input face.
#[derive(Debug)]
pub struct SurfaceMetricWithFalloff<D: DistanceCalculator + Copy + 'static> {
    /// The radius of the sphere.
    pub radius: f32,

    /// The falloff distance of the sphere. This means that the influence
    /// decreases from the radius to the radius + falloff.
    /// The way the influence decreases is controlled by `falloff_func`.
    pub falloff: f32,

    /// The metric squared used to calculate the distance between the input position and the vertices.
    pub metric_squared: D,

    /// The falloff function used to calculate the weight of the selection.
    /// It receives values from 0.0 to 1.0 and has to return a value in the same range.
    /// Simply returning the input value is a linear falloff.
    pub falloff_func: FalloffFn,
}

impl SurfaceMetricWithFalloff<L2> {
    #[inline]
    pub fn sphere(radius: f32, falloff: f32, falloff_func: FalloffFn) -> Self {
        Self {
            radius,
            falloff,
            metric_squared: L2,
            falloff_func,
        }
    }
}

impl<D: DistanceCalculator + Copy + 'static> MeshSelector for SurfaceMetricWithFalloff<D> {
    fn select(
        &self,
        mesh_graph: &MeshGraph,
        input_pos: Vec3,
        input_face: Face,
    ) -> WeightedSelection {
        let sum = self.radius + self.falloff;
        let max_dist_sqr = sum * sum;

        let mut vertices = HashSet::new();
        let mut new_vertices = HashSet::new();

        new_vertices.insert(mesh_graph.halfedges[input_face.halfedge].end_vertex);

        while !new_vertices.is_empty() {
            let mut new_new_vertices = HashSet::new();

            for v_id in new_vertices {
                let pos = mesh_graph.positions[v_id];

                if !vertices.contains(&v_id)
                    && self.metric_squared.distance_squared(pos, input_pos) <= max_dist_sqr
                {
                    vertices.insert(v_id);

                    for he_id in mesh_graph.vertices[v_id].outgoing_halfedges(mesh_graph) {
                        new_new_vertices.insert(mesh_graph.halfedges[he_id].end_vertex);
                    }
                }
            }

            new_vertices = new_new_vertices;
        }

        WeightedSelection {
            selection: Selection {
                faces: faces_incident_to_vertices(vertices, mesh_graph),
                ..Default::default()
            },
            get_weight: get_sphere_with_falloff_weight_callback(
                input_pos,
                self.radius,
                self.falloff,
                self.falloff_func,
                self.metric_squared,
            ),
        }
    }
}
