mod level_loader;

use crate::level_loader::lever_loader::WorldManagementPlugin;
use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
struct Momentum(Vec2);

#[derive(Component)]
struct Gravity(f32);

#[derive(Component, Debug)]
struct Name(String);

#[derive(Component)]
struct StaticSprite(SpriteBundle);

#[derive(Component)]
struct RenderIndex(f32);

#[derive(Component)]
struct PlayerMarker;

#[derive(Bundle)]
struct PlayerBundle {
    marker: PlayerMarker,
    name: Name,
    gravity: Gravity,
    momentum: Momentum,
    render_index: RenderIndex,
}

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PlayerBundle {
            marker: PlayerMarker,
            name: Name("Player".to_string()),
            momentum: Momentum(Vec2::ZERO),
            gravity: Gravity(1.0),
            render_index: RenderIndex(0.0),
        },
        SpriteBundle {
            texture: asset_server.load("player.png"),
            ..default()
        },
    ));
}

fn add_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        // transform: Transform::from_xyz(128.0, 128.0, 0.0),
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::FixedVertical(200.0);
    commands.spawn(camera);
}

fn apply_momentum_system(time: Res<Time>, mut query: Query<(&mut Transform, &Momentum)>) {
    for (mut transform, momentum) in query.iter_mut() {
        let z = transform.translation.z.clone();
        transform.translation += momentum.0.extend(z) * time.delta_seconds();
    }
}

fn apply_gravity_system(time: Res<Time>, mut query: Query<(&mut Momentum, &Gravity)>) {
    for (mut momentum, gravity) in query.iter_mut() {
        momentum.0.y -= gravity.0 * time.delta_seconds();
    }
}

fn camera_move_system(
    mut cam_query: Query<(&mut Transform), (With<Camera>, Without<PlayerMarker>)>,
    player_query: Query<(&Transform), (With<(PlayerMarker)>, Without<Camera>)>,
) {
    let mut camera_transform = cam_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform.translation;
    println!("Camera: {:#?}", camera_transform.translation)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldManagementPlugin)
        .add_systems(Startup, add_player)
        .add_systems(Startup, add_camera)
        .add_systems(Update, apply_momentum_system)
        .add_systems(Update, apply_gravity_system)
        .add_systems(Update, camera_move_system)
        .run();
}
