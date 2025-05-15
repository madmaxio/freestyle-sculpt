use glam::Vec3;
use hashbrown::HashSet;
use parry3d::math::{Point, Vector};

use mesh_graph::{Face, MeshGraph, Selection, SelectionOps};

use super::{MeshSelector, WeightedSelection, get_sphere_with_falloff_weight_callback};

/// Generates a selection of a mesh that is within a sphere with a falloff
pub struct SphereWithFalloff {
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

impl SphereWithFalloff {
    #[inline]
    pub fn new(radius: f32, falloff: f32, falloff_func: fn(f32) -> f32) -> Self {
        Self {
            radius,
            falloff,
            falloff_func,
        }
    }
}

impl MeshSelector for SphereWithFalloff {
    fn select(
        &self,
        mesh_graph: &MeshGraph,
        input_pos: Vec3,
        _input_face: Face,
    ) -> WeightedSelection {
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
