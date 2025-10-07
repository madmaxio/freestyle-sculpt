use glam::Vec3;
use mesh_graph::{MeshGraph, Selection, VertexId, error_none};
use tracing::instrument;

use crate::{
    ray::FaceIntersection,
    selectors::{MeshSelector, WeightedSelection},
};

use super::DeformationField;

/// Smoothing deformation field.
///
/// This deformation field applies a smoothing effect to the selected vertices.
/// It calculates the average position of the surrounding vertices of every selected vertex and moves it towards this average.
pub struct SmoothDeformation {
    selection: Selection,
    weight_callback: Box<dyn Fn(Vec3) -> f32>,
}

impl Default for SmoothDeformation {
    fn default() -> Self {
        Self {
            selection: Selection::default(),
            weight_callback: Box::new(|_| 1.0),
        }
    }
}

impl DeformationField for SmoothDeformation {
    fn on_pointer_move(
        &mut self,
        mesh_graph: &MeshGraph,
        selector: &dyn MeshSelector,
        _mouse_translation: Vec3,
        face_intersection: Option<FaceIntersection>,
    ) -> bool {
        if let Some(FaceIntersection { point, face }) = face_intersection {
            WeightedSelection {
                selection: self.selection,
                get_weight: self.weight_callback,
            } = selector.select(mesh_graph, point, face);

            true
        } else {
            false
        }
    }

    #[instrument(skip(self, mesh_graph))]
    fn vertex_movement(&self, vertex: VertexId, mesh_graph: &MeshGraph) -> Vec3 {
        let mut movement = Vec3::ZERO;

        let neighbours = mesh_graph
            .vertices
            .get(vertex)
            .map(|v| v.neighbours(mesh_graph).collect::<Vec<_>>())
            .or_else(error_none!("Vertex not found"))
            .unwrap_or_default();

        for neighbour in &neighbours {
            movement += mesh_graph
                .positions
                .get(*neighbour)
                .or_else(error_none!("Neighbour position not found"))
                .unwrap_or(&Vec3::ZERO);
        }

        movement /= neighbours.len() as f32;

        let pos = mesh_graph
            .positions
            .get(vertex)
            .or_else(error_none!("Vertex position not found"))
            .unwrap_or(&Vec3::ZERO);
        (movement - pos) * 0.1
    }

    #[inline(always)]
    fn selection(&self) -> &Selection {
        &self.selection
    }

    #[inline(always)]
    fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }

    #[inline(always)]
    fn weight_callback(&self) -> &dyn Fn(Vec3) -> f32 {
        self.weight_callback.as_ref()
    }
}
