//! Renders a 2D scene containing a single, moving sprite.

use std::task::Wake;

use bevy::prelude::*;
use rand::{prelude::StdRng, SeedableRng, RngCore};

// TODO: implement falling
// TODO: implement board structure

const PIECE_SIZE: f32 = 32.;
const RED: Color = Color::rgb(1., 0., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const PURPLE: Color = Color::rgb(0.5, 0., 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, generate_next_piece)
        .run();
}

#[derive(Component, Clone, Copy)]
enum Piece {
    Red,
    Blue,
    Purple,
    Green,
}

impl Piece {
    fn get_color(self) -> Color {
        match self {
            Piece::Red => RED,
            Piece::Blue => BLUE,
            Piece::Green => GREEN,
            Piece::Purple => PURPLE,
        }
    }
}

#[derive(Component)]
struct FallingPair;

#[derive(Component)]
struct Bag { rng: StdRng }

impl Bag {
    fn new() -> Self {
        let rng = StdRng::from_entropy();
        Self { rng }
    }

    fn next_piece(&mut self) -> Piece {
        let r = self.rng.next_u32() % 4;

        match r {
            0 => Piece::Red,
            1 => Piece::Blue,
            2 => Piece::Purple,
            3 => Piece::Green,
            _ => panic!()
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Bag::new());
}

fn generate_next_piece(mut commands: Commands, mut query: Query<&mut Bag>) {
    for mut bag in &mut query {
        let left_piece = bag.next_piece();
        let right_piece = bag.next_piece();

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
                            color: left_piece.get_color(),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(-PIECE_SIZE, 0., 0.),
                            scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    left_piece,
                ));
        
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: right_piece.get_color(),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0., 0., 0.),
                            scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    right_piece,
                ));
            });

    }
}
