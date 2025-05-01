use crate::meshgraph::{FaceId, MeshGraph};

impl MeshGraph {
    /// Deletes a face from the mesh graph together with its edges.
    /// It also deletes the vertices that are no longer connected
    /// to any other faces if `delete_isolated_vertices` is true.
    pub fn delete_face(&mut self, face_id: FaceId, delete_isolated_vertices: bool) {
        let face = self.faces[face_id];

        let vertices = face.vertices(self);
        let halfedges = face.halfedges(self);

        for vertex in &vertices {
            if self.vertices[vertex].halfedges().len()
        }

        self.faces.remove(face_id);
    }
}
