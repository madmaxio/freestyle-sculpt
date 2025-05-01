use bevy::{color::palettes::css::SILVER, prelude::*};
use bevy_panorbit_camera::PanOrbitCamera;
use freestyle_sculpt::meshgraph::{primitives::IcoSphere, MeshGraph};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (mesh, mesh_graph) = init_icosphere();

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
        mesh_graph,
        Name::new("Icosphere"),
        Transform::default(),
    ));

    commands.spawn((
        PointLight {
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 18.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_translation(Vec3::new(0.0, 0.0, 17.0)),
        PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Middle,
            ..default()
        },
    ));
}

fn init_icosphere() -> (Mesh, MeshGraph) {
    let mesh_graph = MeshGraph::from(IcoSphere {
        subdivisions: 2,
        radius: 3.0,
    });

    mesh_graph.log_rerun();

    (mesh_graph.clone().into(), mesh_graph)
}
