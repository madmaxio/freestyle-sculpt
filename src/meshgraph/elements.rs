use glam::Vec3;
use parry3d::partitioning::IndexedData;
use slotmap::new_key_type;

use super::MeshGraph;

new_key_type! { pub struct VertexId; }
new_key_type! { pub struct HalfedgeId; }
new_key_type! { pub struct FaceId; }

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
    /// One of the halfedges with this vertex as start point.
    /// If possible this is a boundary halfedge, i.e. it has no associated face.
    pub outgoing_halfedge: Option<HalfedgeId>,
}

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

#[derive(Default, Debug, Clone, Copy)]
pub struct Face {
    /// One of the halfedges of the face.
    /// Serves as a starting point for traversing the face's edges and vertices
    pub halfedge: HalfedgeId,

    /// The index of the face in the qbvh
    pub index: usize,

    /// The associated face id
    pub id: FaceId,
}

impl IndexedData for Face {
    fn default() -> Self {
        Default::default()
    }

    fn index(&self) -> usize {
        self.index
    }
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
}

impl Face {
    // TODO : create iterator instead of returning a Vec
    /// Returns the three halfedges that form this face
    pub fn halfedges(&self, mesh_graph: &MeshGraph) -> Vec<HalfedgeId> {
        // TODO :this currently only works for triangle meshes
        let mut edges = Vec::with_capacity(3);
        edges.push(self.halfedge);

        let he = mesh_graph.halfedges[self.halfedge];
        let next_id = he.next.unwrap();
        edges.push(next_id);
        edges.push(mesh_graph.halfedges[next_id].next.unwrap());

        edges
    }

    // TODO : create iterator instead of returning a Vec
    /// Returns the three corner vertices of this face.
    pub fn vertices(&self, mesh_graph: &MeshGraph) -> Vec<VertexId> {
        let mut vertices = Vec::with_capacity(3);

        for halfedge in self.halfedges(mesh_graph) {
            vertices.push(mesh_graph.halfedges[halfedge].end_vertex);
        }

        vertices
    }

    /// Center positions of this face.
    pub fn center(&self, mesh_graph: &MeshGraph) -> Vec3 {
        let mut sum = Vec3::ZERO;
        for vertex in self.vertices(mesh_graph) {
            sum += mesh_graph.positions[vertex];
        }
        sum / 3.0
    }
}

impl Vertex {
    /// One of the incoming halfedges of this vertex.
    pub fn incoming_halfedge(&self, mesh_graph: &MeshGraph) -> HalfedgeId {
        mesh_graph.halfedges[self.outgoing_halfedge()].twin()
    }

    #[inline]
    /// One of the outgoing halfedges of this vertex.
    pub fn outgoing_halfedge(&self) -> HalfedgeId {
        self.outgoing_halfedge
            .expect("Outgoing halfedge should be connected")
    }

    // TODO : create iterator instead of returning a Vec
    /// Returns all halfedges that point away from this vertex.
    pub fn outgoing_halfedges(&self, mesh_graph: &MeshGraph) -> Vec<HalfedgeId> {
        let mut edges = Vec::new();

        let mut current_edge = self.outgoing_halfedge();

        edges.push(current_edge);

        while let Some(cw) = mesh_graph.halfedges[current_edge].cw_rotated_neighbour(mesh_graph) {
            if cw == edges[0] {
                break;
            }

            edges.push(cw);
            current_edge = cw;
        }

        edges
    }

    /// Returns all halfedges that point towards this vertex
    pub fn incoming_halfedges(&self, mesh_graph: &MeshGraph) -> Vec<HalfedgeId> {
        self.outgoing_halfedges(mesh_graph)
            .into_iter()
            .map(|he_id| mesh_graph.halfedges[he_id].twin())
            .collect()
    }

    // TODO : create iterator instead of returning a Vec
    /// Returns all faces incident to this vertex.
    pub fn faces(&self, mesh_graph: &MeshGraph) -> Vec<FaceId> {
        self.outgoing_halfedges(mesh_graph)
            .into_iter()
            .filter_map(|he| mesh_graph.halfedges[he].face)
            .collect()
    }

    // TODO : create iterator instead of returning a Vec
    /// Returns all neighbouring (connected through an edge) vertices of this vertex.
    pub fn neighbours(&self, mesh_graph: &MeshGraph) -> Vec<VertexId> {
        self.outgoing_halfedges(mesh_graph)
            .into_iter()
            .map(|he| mesh_graph.halfedges[he].end_vertex)
            .collect()
    }

    /// The degree of this vertex, i.e., the number of edges incident to it. Sometimes called the valence.
    #[inline]
    pub fn degree(&self, mesh_graph: &MeshGraph) -> usize {
        self.neighbours(mesh_graph).len()
    }
}
