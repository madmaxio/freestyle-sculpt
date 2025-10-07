use glam::{Vec3, vec3};
use parry3d::{
    math::{Point, Vector},
    query::{PointQueryWithLocation, RayCast},
};

use mesh_graph::{Face, MeshGraph};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Returns the point on the ray `origin + toi * direction`
    pub fn point_at(self, toi: f32) -> Vec3 {
        self.origin + toi * self.direction
    }

    /// Casts the ray and returns the face id and point of the first intersection.
    pub fn cast_ray_and_get_face_id(self, mesh_graph: &MeshGraph) -> Option<FaceIntersection> {
        let parry_ray = self.into();

        let toi = mesh_graph.cast_local_ray(&parry_ray, f32::MAX, true)?;
        let hit_point = parry_ray.point_at(toi);

        // TODO : implement cast_local_ray_and_get_location so this is not necessary
        let (_, face) = mesh_graph.project_local_point_and_get_location_with_max_dist(
            &hit_point,
            true,
            f32::MAX,
        )?;

        Some(FaceIntersection {
            point: vec3(hit_point.x, hit_point.y, hit_point.z),
            face,
        })
    }
}

impl From<Ray> for parry3d::query::Ray {
    fn from(ray: Ray) -> Self {
        Self::new(
            Point::new(ray.origin.x, ray.origin.y, ray.origin.z),
            Vector::new(ray.direction.x, ray.direction.y, ray.direction.z),
        )
    }
}

/// Used with pointer-mesh interactions.
/// - **`point`** is the point on the mesh, that has been clicked or touched.
///   This is is usually found by ray casting.
/// - **`face`** is the face on the mesh, that has been clicked or touched.
///   This is also usually found by ray casting.
#[derive(Debug, Clone, Copy)]
pub struct FaceIntersection {
    pub point: Vec3,
    pub face: Face,
}
