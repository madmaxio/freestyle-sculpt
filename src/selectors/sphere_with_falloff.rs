use glam::Vec3;
use hashbrown::HashSet;
use parry3d::math::{Point, Vector};

use crate::meshgraph::{Face, MeshGraph, Selection, SelectionOps};

use super::{get_sphere_with_falloff_weight_callback, MeshSelector};

/// Generates a selection on the surface of a mesh that is within a sphere with a falloff and that
/// is limited to be connected to the input face.
pub struct SphereWithFalloff {
    /// The radius of the sphere.
    radius: f32,
    /// The falloff distance of the sphere. This means that the influence
    /// decreases linearly from the radius to the radius + falloff.
    falloff: f32,
}

impl SphereWithFalloff {
    pub fn new(radius: f32, falloff: f32) -> Self {
        Self { radius, falloff }
    }
}

impl MeshSelector for SphereWithFalloff {
    fn select(
        &self,
        mesh_graph: &MeshGraph,
        input_pos: Vec3,
        _input_face: Face,
    ) -> (Selection, Box<dyn Fn(Vec3) -> f32>) {
        let mut selection = Selection::default();

        let mut potential_faces = vec![];

        let aabb = parry3d::bounding_volume::Aabb::from_half_extents(
            Point::new(input_pos.x, input_pos.y, input_pos.z),
            Vector::from_element(self.radius + self.falloff),
        );
        mesh_graph.qbvh.intersect_aabb(&aabb, &mut potential_faces);

        let potential_selection = Selection {
            faces: HashSet::from_iter(potential_faces.into_iter().map(|f| f.id)),
            ..Default::default()
        };

        let sum = self.radius + self.falloff;
        let max_dist_sqr = sum * sum;

        for vertex_id in potential_selection.resolve_to_vertices(mesh_graph) {
            let distance = mesh_graph.positions[vertex_id].distance_squared(input_pos);

            if distance <= max_dist_sqr {
                selection.insert(vertex_id);
            }
        }

        (
            selection,
            get_sphere_with_falloff_weight_callback(input_pos, self.radius, self.falloff),
        )
    }
}
