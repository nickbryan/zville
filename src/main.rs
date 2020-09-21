use bevy::prelude::*;

mod camera;
mod cursor;
mod voxel;
mod window;

fn main() {
    App::build()
        .add_plugin(window::WindowPlugin)
        .add_default_plugins()
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_plugin(cursor::CursorPlugin)
        .add_plugin(voxel::VoxelPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxel_matrices: ResMut<Assets<voxel::Matrix>>,
) {
    let ground_handle = asset_server
        .load_sync(&mut voxel_matrices, "assets/ground.qb")
        .unwrap();

    let ground = voxel_matrices.get(&ground_handle).unwrap();

    for (mesh, color) in ground.mesh_parts() {
        let handle = meshes.add(mesh);
        commands.spawn(PbrComponents {
            mesh: handle,
            material: materials.add(StandardMaterial {
                albedo: color,
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(0.0, 250.0, 0.0)),
        ..Default::default()
    });
}
