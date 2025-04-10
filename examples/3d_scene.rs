use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_where_was_i::{WhereWasI, WhereWasIPlugin};

/// Example for a 3D scene with a circular base and a cube.
///
/// This example saves the camera transform and restores it.
///
/// Based on https://bevyengine.org/examples/3d-rendering/3d-scene/
fn main() {
    App::new()
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(WhereWasIPlugin {
            directory: "./saves".into(),
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_camera(mut commands: Commands<'_, '_>) {
    commands.spawn((
        Camera::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera {
            button_pan: MouseButton::Middle,
            button_orbit: MouseButton::Left,
            ..Default::default()
        },
        WhereWasI {
            name: "camera".into(),
        },
    ));
}

fn setup_scene(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
