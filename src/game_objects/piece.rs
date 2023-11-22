use std::task::Wake;

use bevy::sprite::collide_aabb::collide;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::collide_aabb::Collision,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::game_objects::movement::{Collider, CollisionEvent, Fall, FallState};

const PIECE_SIZE: f32 = 32.;
const PIECE_FALL_SPEED: f32 = 150.;
const PIECE_LATERAL_SPEED: f32 = 400.;

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

#[derive(Component, Debug)]
pub struct Piece;

#[derive(Component)]
pub struct Controllable {
    stuck_left: bool,
    stuck_right: bool,
}

#[derive(Event, Default)]
pub struct NextPieceEvent;

impl Controllable {
    pub fn new() -> Self {
        Self {
            stuck_left: false,
            stuck_right: false,
        }
    }
}

#[derive(Bundle)]
pub struct PieceBundle {
    color: PieceColor,
    sprite_bundle: SpriteBundle,
    fall: Fall,
    piece: Piece,
    collider: Collider,
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
            piece: Piece,
            collider: Collider,
        }
    }
}

pub fn check_for_collisions(
    mut fall_query: Query<(Entity, &Transform, &mut Fall, Option<&mut Controllable>)>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    for (entity, mut fall_transform, mut fall, mut maybe_controllable) in fall_query.iter_mut() {
        if let Some(ref mut controllable) = maybe_controllable {
            controllable.stuck_right = false;
            controllable.stuck_left = false;
        }
        for collider_transform in collider_query.iter() {
            let collision = collide(
                fall_transform.translation,
                vec2(fall_transform.scale.x, fall_transform.scale.y),
                collider_transform.translation,
                vec2(collider_transform.scale.x, collider_transform.scale.y),
            );

            if let Some(collision) = collision {
                let mut offset = (fall_transform.scale + collider_transform.scale) * 0.5
                    - (fall_transform.translation - collider_transform.translation).abs();
                offset.z = 0.;
                collision_event.send(CollisionEvent::new(entity, offset, collision));
            }
        }
    }
}

pub fn handle_collisions(
    mut event_reader: EventReader<CollisionEvent>,
    mut query: Query<(&mut Transform, &mut Fall, Option<&mut Controllable>)>,
    mut next_piece_event: EventWriter<NextPieceEvent>,
) {
    for event in event_reader.read() {
        if let Ok((mut transform, mut fall, mut maybe_controllable)) = query.get_mut(event.entity) {
            match event.collision {
                Collision::Top => {
                    fall.state = FallState::Stopped;
                    if maybe_controllable.is_some() {
                        next_piece_event.send_default();
                    }
                    transform.translation.y += event.offset.y;
                }
                Collision::Left => {
                    if let Some(ref mut controllable) = maybe_controllable {
                        controllable.stuck_right = true;
                    }
                    transform.translation.x -= event.offset.x;
                }
                Collision::Right => {
                    if let Some(ref mut controllable) = maybe_controllable {
                        controllable.stuck_left = true;
                    }
                    transform.translation.x += event.offset.x;
                }
                _ => (),
            }
        }
    }
}

pub fn spawn_next_piece(
    mut commands: Commands,
    mut event_reader: EventReader<NextPieceEvent>,
    mut query_bag: Query<&mut Bag>,
    mut query_active: Query<(Entity, &mut Controllable)>,
) {
    let mut flag = false;
    for _ in event_reader.read() {
        flag = true;
    }
    if flag {
        let (active_entity, _) = query_active.single_mut();
        commands.entity(active_entity).remove::<Controllable>();
        let mut bag = query_bag.single_mut();
        commands.spawn((bag.new_piece(vec3(0., 200., 0.)), Controllable::new()));
    }
}

pub fn move_active_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query_active_piece: Query<(&mut Transform, &mut Controllable, &mut Fall)>,
    time: Res<Time>,
) {
    let (mut transform, mut controllable, mut fall) = query_active_piece.single_mut();
    if keyboard_input.pressed(KeyCode::Left) && !controllable.stuck_left {
        transform.translation.x -= PIECE_LATERAL_SPEED * time.delta_seconds();
        controllable.stuck_right = false;
    }
    if keyboard_input.pressed(KeyCode::Right) && !controllable.stuck_right {
        transform.translation.x += PIECE_LATERAL_SPEED * time.delta_seconds();
        controllable.stuck_left = false;
    }

    if let FallState::Normal | FallState::Fast = fall.state {
        if keyboard_input.pressed(KeyCode::Down) {
            fall.state = FallState::Fast;
        } else {
            fall.state = FallState::Normal;
        }
    }
}
