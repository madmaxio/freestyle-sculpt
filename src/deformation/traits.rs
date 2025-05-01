use glam::Vec3;

use crate::{
    meshgraph::{MeshGraph, Selection, VertexId},
    ray::FaceIntersection,
    selectors::MeshSelector,
};

use super::SculptParams;

pub trait DeformationField {
    fn vertex_movement(&self, vertex: VertexId, mesh_graph: &MeshGraph) -> Vec3;

    fn on_pointer_down(
        &mut self,
        _mesh_graph: &MeshGraph,
        _selector: &dyn MeshSelector,
        _face_intersection: FaceIntersection,
    ) {
        // by default, do nothing
    }

    fn on_pointer_move(
        &mut self,
        _mesh_graph: &MeshGraph,
        _selector: &dyn MeshSelector,
        _mouse_translation: Vec3,
        _face_intersection: Option<FaceIntersection>,
    ) -> bool {
        // by default, do nothing
        true
    }

    fn selection(&self) -> &Selection;

    fn selection_mut(&mut self) -> &mut Selection;

    fn weight_callback(&self) -> &dyn Fn(Vec3) -> f32;

    fn max_movement_squared(&self, mesh_graph: &MeshGraph, strength: f32) -> f32 {
        let affected_vertices = self.selection().resolve_to_vertices(mesh_graph);
        let get_weight = self.weight_callback();

        let mut max_movement_squared: f32 = 0.0;

        for vertex in &affected_vertices {
            let movement = self.vertex_movement(*vertex, mesh_graph)
                * get_weight(mesh_graph.positions[*vertex])
                * strength;
            max_movement_squared = max_movement_squared.max(movement.length_squared());
        }

        max_movement_squared
    }

    fn apply(&mut self, mesh_graph: &mut MeshGraph, strength: f32, params: SculptParams) {
        let max_movement_squared = self.max_movement_squared(mesh_graph, strength);

        let steps = (max_movement_squared / params.max_move_dist_squared)
            .sqrt()
            .ceil();

        let factor = 1.0 / steps;

        let selection = self.selection_mut();

        #[cfg(feature = "rerun")]
        {
            mesh_graph.log_rerun();
        }

        mesh_graph.collapse_until_edges_above_min_length(params.min_edge_length_squared, selection);

        mesh_graph
            .subdivide_until_edges_below_max_length(params.max_edge_length_squared, selection);

        let mut movements = Vec::new();

        for _ in 0..steps as usize {
            let affected_vertices = self.selection().resolve_to_vertices(mesh_graph);
            movements.clear();

            let get_weight = self.weight_callback();

            for vertex in &affected_vertices {
                let movement = self.vertex_movement(*vertex, mesh_graph)
                    * get_weight(mesh_graph.positions[*vertex])
                    * strength;
                movements.push(movement);
            }

            let selection = self.selection_mut();

            for (vertex, movement) in affected_vertices.iter().zip(movements.iter()) {
                mesh_graph.positions[*vertex] += *movement * factor;
            }

            #[cfg(feature = "rerun")]
            {
                mesh_graph.log_rerun();
            }

            mesh_graph
                .collapse_until_edges_above_min_length(params.min_edge_length_squared, selection);

            mesh_graph
                .subdivide_until_edges_below_max_length(params.max_edge_length_squared, selection);

            // TODO : merging and separation and cleanup
        }

        mesh_graph.refit_qbvh();
    }
}
