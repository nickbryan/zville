use bevy::prelude::*;
use bevy_mod_picking::*;

mod camera;
mod cursor;
mod voxel;
mod window;

fn main() {
    App::build()
        .add_plugin(window::Plugin)
        .add_default_plugins()
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_plugin(cursor::Plugin)
        .add_plugin(voxel::Plugin)
        .add_plugin(camera::Plugin)
        .add_plugin(PickingPlugin)
        .add_startup_stage_after(camera::STARTUP_STAGE, "main")
        .add_startup_system_to_stage("main", setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_pick_group: Res<camera::CameraPickingGroup>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxel_matrices: ResMut<Assets<voxel::Matrix>>,
) {
    let ground_handle = asset_server
        .load_sync(&mut voxel_matrices, "assets/ground.qb")
        .unwrap();

    let small_model_handle = asset_server
        .load_sync(&mut voxel_matrices, "assets/16x16x16.qb")
        .unwrap();

    let ground = voxel_matrices.get(&ground_handle).unwrap();
    let small_model = voxel_matrices.get(&small_model_handle).unwrap();

    for (mesh, color) in ground.mesh_parts() {
        let handle = meshes.add(mesh);
        commands
            .spawn(PbrComponents {
                mesh: handle,
                material: materials.add(StandardMaterial {
                    albedo: color,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .with(PickableMesh::new([camera_pick_group.0].into()));
    }

    for (mesh, color) in small_model.mesh_parts() {
        let handle = meshes.add(mesh);
        commands
            .spawn(PbrComponents {
                transform: Transform::from_translation_rotation_scale(
                    Vec3::new(10.0, 5.0, 10.0),
                    Quat::identity(),
                    1.0 / 16.0,
                ),
                mesh: handle,
                material: materials.add(StandardMaterial {
                    albedo: color,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .with(PickableMesh::new([camera_pick_group.0].into()));
    }

    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(0.0, 250.0, 0.0)),
        ..Default::default()
    });
}
