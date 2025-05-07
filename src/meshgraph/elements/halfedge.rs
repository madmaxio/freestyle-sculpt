use crate::meshgraph::MeshGraph;

use super::{FaceId, HalfedgeId, VertexId};

#[derive(Debug, Clone, Copy)]
pub struct Halfedge {
    /// The vertex that this halfedge points to.
    pub end_vertex: VertexId,

    /// The face associated to this halfedge. `None` if this is a boundary halfedge.
    pub face: Option<FaceId>,

    /// This is the halfedge opposite.
    /// It points backwards compared to this halfedge (from end_vertex to start_vertex).
    pub twin: Option<HalfedgeId>,

    /// The next halfedge in the face. `None` if this is a boundary halfedge.
    pub next: Option<HalfedgeId>,
}

impl Halfedge {
    /// Start vertex from which this halfedge points away
    pub fn start_vertex(&self, mesh_graph: &MeshGraph) -> VertexId {
        mesh_graph.halfedges[self.twin()].end_vertex
    }

    /// Same edge but points the opposite way. Belongs to the face that adjacent to the
    /// face of this halfedge.
    #[inline]
    pub fn twin(&self) -> HalfedgeId {
        self.twin.expect("Twin should be connected by now")
    }

    /// Previous halfedge that shares the same face
    pub fn prev(&self, mesh_graph: &MeshGraph) -> Option<HalfedgeId> {
        // TODO : this only works for triangle meshes
        self.next
            .map(|next_id| mesh_graph.halfedges[next_id].next.unwrap())
    }

    /// In counter-clockwise order next halfedge that has the same start vertex
    pub fn ccw_rotated_neighbour(&self, mesh_graph: &MeshGraph) -> Option<HalfedgeId> {
        self.prev(mesh_graph)
            .map(|prev| mesh_graph.halfedges[prev].twin())
    }

    /// In clockwise order next halfedge that has the same start vertex
    pub fn cw_rotated_neighbour(&self, mesh_graph: &MeshGraph) -> Option<HalfedgeId> {
        mesh_graph.halfedges[self.twin()].next
    }

    /// Length of the halfedge squared.
    pub fn length_squared(&self, mesh_graph: &MeshGraph) -> f32 {
        let start = mesh_graph.positions[self.start_vertex(mesh_graph)];
        let end = mesh_graph.positions[self.end_vertex];

        start.distance_squared(end)
    }

    /// Returns `true` if there is no face adjacent to this halfedge.
    #[inline]
    pub fn is_boundary(&self) -> bool {
        self.face.is_none()
    }

    pub fn is_adjacent_to_boundary(&self, mesh_graph: &MeshGraph) -> bool {
        mesh_graph.vertices[self.start_vertex(mesh_graph)].is_boundary(mesh_graph)
            || mesh_graph.vertices[self.end_vertex].is_boundary(mesh_graph)
    }
}
