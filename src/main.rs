//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, piece_movement)
        .run();
}

#[derive(Component)]
enum Piece {
    Red,
    Blue,
    Purple,
    Green,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Piece::Red,
        SpriteBundle {
            texture: asset_server.load("sprites/red.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        }
    ));
}

fn piece_movement(time: Res<Time>, mut sprite_position: Query<(&mut Piece, &mut Transform)>) {
    for (_, mut transform) in &mut sprite_position {
        transform.translation.y -= 150. * time.delta_seconds();
    }
}
