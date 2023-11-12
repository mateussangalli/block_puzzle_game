//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

const PIECE_SIZE: f32 = 32.;
const RED: Color = Color::rgb(1., 0., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const PURPLE: Color = Color::rgb(0.5, 0., 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
enum Piece {
    Red,
    Blue,
    Purple,
    Green,
}

#[derive(Component)]
struct FallingPair;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            FallingPair,
            SpatialBundle {
                transform: Transform::IDENTITY,
                ..default()
            }
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: RED,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(-PIECE_SIZE, 0., 0.),
                        scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Piece::Red,
            ));

            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: PURPLE,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0., 0., 0.),
                        scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Piece::Purple,
            ));
        });
}
