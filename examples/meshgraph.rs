use freestyle_sculpt::{
    meshgraph::{primitives::IcoSphere, MeshGraph, Selection},
    RR,
};

pub fn main() {
    RR.log_static("/", &rerun::ViewCoordinates::RIGHT_HAND_Y_UP())
        .unwrap();

    // let mut mesh = MeshGraph::from(Quad(
    //     vec3(0.0, 0.0, 0.0),
    //     vec3(10.0, 0.0, 0.0),
    //     vec3(10.0, 10.0, 0.0),
    //     vec3(0.0, 10.0, 0.0),
    // ));

    let mut mesh = MeshGraph::from(IcoSphere {
        radius: 10.0,
        subdivisions: 2,
    });

    mesh.log_rerun();

    let mut selection = Selection::select_all(&mesh);
    mesh.subdivide_until_edges_below_max_length(4.0 * 4.0, &mut selection);

    mesh.collapse_until_edges_above_min_length(3.0 * 3.0, &mut selection);

    RR.flush_blocking()
}
