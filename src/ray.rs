use glam::{vec3, Vec3};
use parry3d::{
    math::{Point, Vector},
    query::{PointQueryWithLocation, RayCast},
};

use crate::meshgraph::{Face, MeshGraph};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(self, toi: f32) -> Vec3 {
        self.origin + toi * self.direction
    }

    pub fn cast_ray_and_get_face_id(self, graph: &MeshGraph) -> Option<FaceIntersection> {
        let parry_ray = self.into();

        graph.cast_local_ray(&parry_ray, f32::MAX, true).map(|toi| {
            let hit_point = parry_ray.point_at(toi);

            // TODO : implement cast_local_ray_and_get_location so this is not necessary
            let (_, (face, _)) = graph.project_local_point_and_get_location(&hit_point, true);

            FaceIntersection {
                point: vec3(hit_point.x, hit_point.y, hit_point.z),
                face,
            }
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

#[derive(Debug, Clone, Copy)]
pub struct FaceIntersection {
    pub point: Vec3,
    pub face: Face,
}
