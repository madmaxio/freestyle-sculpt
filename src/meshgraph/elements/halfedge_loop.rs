use derive_more::{Deref, DerefMut};
use hashbrown::{HashMap, HashSet};

use crate::meshgraph::MeshGraph;

use super::{HalfedgeId, VertexId};

// TODO : Implement as iterator
/// Halfedge loop ordered counter-clockwise.
///
/// It is not necessarily completely closed or connected. But it is not self-intersecting.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct HalfedgeLoop(Vec<HalfedgeId>);

#[derive(Debug, Clone)]
pub struct BiggerLoop {
    pub grown_loop: HalfedgeLoop,
    pub visited_vertices: HashSet<VertexId>,
}

impl HalfedgeLoop {
    /// Create a new halfedge loop from any `HalfedgeId` container that can be iterated over.
    ///
    /// This will not check if this is a valid loop, so make sure you only use this properly.
    #[inline]
    pub fn new_unchecked(halfedges: impl IntoIterator<Item = HalfedgeId>) -> Self {
        HalfedgeLoop(Vec::from_iter(halfedges))
    }

    /// Extend the loop outward by one face
    pub fn bigger_loop(&self, mesh_graph: &MeshGraph) -> BiggerLoop {
        #[cfg(feature = "rerun")]
        {
            crate::RR
                .log("meshgraph/halfedge/bigger_loop", &rerun::Clear::recursive())
                .unwrap();
            mesh_graph.log_hes_rerun("bigger_loop/current", &self.0);
        }

        let mut visited_vertices = HashSet::new();
        let mut preprocessed_loop = self.0.clone();

        loop {
            let (new_loop, vertices) =
                Self::without_concave_corners(&preprocessed_loop, mesh_graph);

            if new_loop.len() == preprocessed_loop.len() {
                break;
            }

            preprocessed_loop = new_loop;
            visited_vertices = visited_vertices.union(&vertices).copied().collect();
        }

        #[cfg(feature = "rerun")]
        mesh_graph.log_hes_rerun("bigger_loop/preprocessed", &preprocessed_loop);

        let mut prev_twin =
            mesh_graph.halfedges[preprocessed_loop[preprocessed_loop.len() - 1]].twin();

        let mut new_loop = Vec::with_capacity(preprocessed_loop.len());

        for he_id in preprocessed_loop {
            #[cfg(feature = "rerun")]
            {
                mesh_graph.log_he_rerun("bigger_loop/he", he_id);
                mesh_graph.log_he_rerun("bigger_loop/prev_twin", prev_twin);
            }

            let he = mesh_graph.halfedges[he_id];

            // Loop clockwise until you hit the previous edge
            if let Some(mut neighbor_id) = he.cw_rotated_neighbour(mesh_graph) {
                #[cfg(feature = "rerun")]
                mesh_graph.log_he_rerun("bigger_loop/cw_neighbor", neighbor_id);

                let mut loop_segment = Vec::new();
                while let Some(next_neighbor_id) =
                    mesh_graph.halfedges[neighbor_id].cw_rotated_neighbour(mesh_graph)
                {
                    visited_vertices.insert(mesh_graph.halfedges[neighbor_id].end_vertex);

                    if next_neighbor_id == prev_twin {
                        break;
                    }

                    if let Some(next) = mesh_graph.halfedges[next_neighbor_id].next {
                        #[cfg(feature = "rerun")]
                        mesh_graph.log_he_rerun("bigger_loop/added", next);

                        loop_segment.push(next);
                    }

                    neighbor_id = next_neighbor_id;

                    #[cfg(feature = "rerun")]
                    mesh_graph.log_he_rerun("bigger_loop/cw_neighbor", neighbor_id);
                }

                loop_segment.reverse();

                new_loop.extend(loop_segment);
            }

            prev_twin = he.twin();
        }

        #[cfg(feature = "rerun")]
        mesh_graph.log_hes_rerun("bigger_loop/result_with_self_intersections", &new_loop);

        let mut grown_loop = Self::new_unchecked(new_loop);
        grown_loop.remove_self_intersections(mesh_graph);

        #[cfg(feature = "rerun")]
        mesh_graph.log_hes_rerun("bigger_loop/result_cleaned", &grown_loop.0);

        BiggerLoop {
            grown_loop,
            visited_vertices,
        }
    }

    pub fn remove_self_intersections(&mut self, mesh_graph: &MeshGraph) {
        let mut visited_vertices = HashMap::<VertexId, usize>::new();

        let mut i = 0;

        while i < self.len() {
            let he = mesh_graph.halfedges[self[i]];
            let end_v_id = he.end_vertex;

            if let Some(prev_index) = visited_vertices.get(&end_v_id) {
                self.drain(*prev_index + 1..=i);

                i = *prev_index;

                visited_vertices.retain(|_, idx| *idx <= i);
            } else {
                visited_vertices.insert(end_v_id, i);
            }

            i += 1;
        }
    }

    fn without_concave_corners(
        input_loop: &[HalfedgeId],
        mesh_graph: &MeshGraph,
    ) -> (Vec<HalfedgeId>, HashSet<VertexId>) {
        // two copies of self so we can wrap around
        let mut input = input_loop.to_vec();
        input.extend(input_loop);
        input.reverse();

        let mut result = Vec::with_capacity(input_loop.len());
        let mut visited_vertices = HashSet::new();

        let mut prev_twin = mesh_graph.halfedges[input[0]].twin();

        while let Some(he_id) = input.pop() {
            #[cfg(feature = "rerun")]
            {
                mesh_graph.log_he_rerun("bigger_loop/wo_concave/he", he_id);
                mesh_graph.log_he_rerun("bigger_loop/wo_concave/prev_twin", prev_twin);
            }

            let mut next = he_id;

            let mut i = 1;
            let mut jumped_to_i = None;

            let mut neighbor_id = he_id;
            while let Some(next_neighbor_id) =
                mesh_graph.halfedges[neighbor_id].cw_rotated_neighbour(mesh_graph)
            {
                #[cfg(feature = "rerun")]
                mesh_graph.log_he_rerun("bigger_loop/wo_concave/neighbor", next_neighbor_id);

                if next_neighbor_id == prev_twin {
                    break;
                }

                let neighbor_vertex_id = mesh_graph.halfedges[next_neighbor_id].end_vertex;

                visited_vertices.insert(neighbor_vertex_id);

                if let Some(peek) = input.get(input.len() - i) {
                    if mesh_graph.halfedges[*peek].end_vertex == neighbor_vertex_id {
                        #[cfg(feature = "rerun")]
                        mesh_graph.log_he_rerun("bigger_loop/wo_concave/found", next_neighbor_id);

                        next = next_neighbor_id;
                        jumped_to_i = Some(i);
                    }
                } else {
                    break;
                }

                i += 1;
                neighbor_id = next_neighbor_id;
            }

            #[cfg(feature = "rerun")]
            mesh_graph.log_he_rerun("bigger_loop/wo_concave/next", next);

            result.push(next);

            if let Some(jumped_to_i) = jumped_to_i {
                for _ in 0..jumped_to_i {
                    #[cfg(not(feature = "rerun"))]
                    input.pop();

                    #[cfg(feature = "rerun")]
                    {
                        let remove = input.pop().unwrap();
                        mesh_graph.log_he_rerun("bigger_loop/wo_concave/remove", remove);
                    }
                }
            }

            let len_diff = input_loop.len() as i32 - input.len() as i32;

            if len_diff >= 0 {
                // we have completed the loop. now let's see if we added some
                // halfedges at the start that were skipped when we wrapped around

                for _ in 0..len_diff {
                    result.remove(0);
                }

                break;
            }

            prev_twin = mesh_graph.halfedges[next].twin();
        }

        (result, visited_vertices)
    }
}
