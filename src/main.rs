//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use rand::{prelude::StdRng, RngCore, SeedableRng};

// TODO: implement falling
// TODO: implement board structure

const PIECE_SIZE: f32 = 32.;
const RED: Color = Color::rgb(1., 0., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const PURPLE: Color = Color::rgb(0.5, 0., 0.5);
const DOWNWARD_SPEED: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, generate_next_piece)
        .add_systems(Update, update_falling_pair)
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
struct Bag {
    rng: StdRng,
}

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
            _ => panic!(),
        }
    }

    fn spawn_piece(&mut self, builder: &mut ChildBuilder, position: Vec3) {
        let piece = self.next_piece();
        builder.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: piece.get_color(),
                    ..default()
                },
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                    ..default()
                },
                ..default()
            },
            piece,
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Bag::new());
    println!("TT");
}

fn generate_next_piece(mut commands: Commands, mut query: Query<&mut Bag>) {
    dbg!(&query);
    let mut bag = query.single_mut();
    println!("spawn");
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::IDENTITY,
                ..default()
            },
            FallingPair,
        ))
        .with_children(|parent| {
            bag.spawn_piece(parent, Vec3::new(-PIECE_SIZE, 0., 0.));
            bag.spawn_piece(parent, Vec3::new(0., 0., 0.));
        });
    
}

fn update_falling_pair(mut query: Query<(&mut Transform, &FallingPair)>, time: Res<Time>) {
    if let Some((mut transform, falling_pair)) = query.iter_mut().next() {
        transform.translation.y -= DOWNWARD_SPEED * time.delta_seconds();
    }

}
