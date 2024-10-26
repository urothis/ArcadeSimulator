use avian3d::{prelude::*, PhysicsPlugins};
use bevy::{color::palettes::tailwind, input::common_conditions::input_toggle_active, prelude::*};
use building::prelude::*;
use iyes_perf_ui::entries::PerfUiBundle;
use player::PlayerControllerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
            SimulatorBuildingPlugin,
            PlayerControllerPlugin,
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
            material: materials.add(Color::from(tailwind::GRAY_300)),
            transform: Transform {
                translation: Vec3::new(0.0, -1.0, 0.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Static,
        ColliderConstructor::ConvexHullFromMesh
    ));

    commands.spawn(PerfUiBundle::default());
}
