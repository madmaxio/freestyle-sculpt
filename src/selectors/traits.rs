use glam::Vec3;

use crate::meshgraph::{Face, MeshGraph, Selection};

pub trait MeshSelector {
    fn select(
        &self,
        mesh_graph: &MeshGraph,
        input_pos: Vec3,
        input_face: Face,
    ) -> (Selection, Box<dyn Fn(Vec3) -> f32>);
}
