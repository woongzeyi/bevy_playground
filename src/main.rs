use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0.0, 0.0, 4.0)
                .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), -PI / 10.0)),
            ..default()
        },
        Shape,
    ));
    // commands.spawn((PbrBundle {
    //     mesh: meshes.add(shape::Plane { size: 50. }.into()),
    //     material: materials.add(Color::BLACK.into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 8.0)
    //         .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
    //     ..default()
    // },));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 100.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 4.0), Vec3::Y),
        ..default()
    });
    let funky = asset_server.load("funky_town.ogg");
    audio.play_with_settings(funky, PlaybackSettings::LOOP);
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_local_y(time.delta_seconds());
    }
}
