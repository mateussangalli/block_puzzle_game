use bevy::{prelude::*, math::vec2, sprite::collide_aabb::Collision};
use bevy::sprite::collide_aabb::collide;
use rand::{rngs::StdRng, SeedableRng, RngCore};

use crate::game_objects::movement::{Fall, Collider};

const PIECE_SIZE: f32 = 32.;
const PIECE_FALL_SPEED: f32 = 100.;

const RED: Color = Color::rgb(1., 0., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const PURPLE: Color = Color::rgb(0.5, 0., 0.5);

#[derive(Component, Clone, Copy)]
pub enum PieceColor {
    Red,
    Blue,
    Purple,
    Green,
}

impl PieceColor {
    fn get_color(self) -> Color {
        match self {
            PieceColor::Red => RED,
            PieceColor::Blue => BLUE,
            PieceColor::Green => GREEN,
            PieceColor::Purple => PURPLE,
        }
    }
}

#[derive(Component)]
pub struct Piece;

#[derive(Bundle)]
pub struct PieceBundle {
    color: PieceColor,
    sprite_bundle: SpriteBundle,
    fall: Fall,
    piece: Piece
}



#[derive(Component)]
pub struct Bag {
    rng: StdRng,
}

impl Bag {
    pub fn new() -> Self {
        let rng = StdRng::from_entropy();
        Self { rng }
    }

    fn piece_color(&mut self) -> PieceColor {
        let r = self.rng.next_u32() % 4;

        match r {
            0 => PieceColor::Red,
            1 => PieceColor::Blue,
            2 => PieceColor::Purple,
            3 => PieceColor::Green,
            _ => panic!(),
        }
    }

    pub fn new_piece(&mut self, position: Vec3) -> PieceBundle {
        let color = self.piece_color();
        PieceBundle {
            color,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: color.get_color(),
                    ..default()
                },
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                    ..default()
                },
                ..default()
            },
            fall: Fall::new(PIECE_FALL_SPEED),
            piece: Piece
        }
    }
}

pub fn check_for_collisions(
    mut fall_query: Query<(&Transform, &mut Fall), With<Piece>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    for (fall_transform, mut fall) in fall_query.iter_mut() {
        for collider_transform in collider_query.iter() {
            let collision = collide(
                fall_transform.translation,
                vec2(fall_transform.scale.x, fall_transform.scale.y),
                collider_transform.translation,
                vec2(collider_transform.scale.x, collider_transform.scale.y),
            );
    
            if let Some(Collision::Top) = collision {
                fall.stop();
            }
        }
    }
}
