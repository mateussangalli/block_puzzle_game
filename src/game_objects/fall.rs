use std::task::Wake;

use bevy::{prelude::*, math::vec2, sprite::collide_aabb::Collision};
use bevy::sprite::collide_aabb::collide;

use crate::game_objects::{grid::{GridPosition, GameGrid}, piece::{Controllable, NextPieceEvent, Piece}};

const FAST_MULT: f32 = 3.;

#[derive(Component)]
pub struct Fall {
    velocity: f32,
    pub state: FallState
}

#[derive(Clone, Copy)]
pub enum FallState {
    Fast,
    Normal,
    Stopped
}

impl Fall {
    pub fn new(velocity: f32) -> Self {
        Self {
            velocity,
            state: FallState::Normal
        }
    }

    pub fn get_velocity(&self) -> f32 {
        match self.state {
            FallState::Fast => self.velocity * FAST_MULT,
            FallState::Normal => self.velocity,
            FallState::Stopped => 0.
        }
    }
}

pub fn update_fall(
    mut query_piece: Query<(&mut Transform, &mut GridPosition, &mut Fall, &Piece, Option<&Controllable>)>,
    mut query_grid: Query<&mut GameGrid>,
    time: Res<Time>,
    mut next_piece_event: EventWriter<NextPieceEvent>,
) {
    let mut grid = query_grid.single_mut();

    for (mut transform, mut position, mut fall, piece, maybe_controllable) in query_piece.iter_mut() {
        transform.translation.y -= time.delta_seconds() * fall.get_velocity();
        *position = grid.vec3_to_position(transform.translation);
        let discretized_position = grid.position_to_vec3(*position);
        if !grid.can_move_down(*position) && (transform.translation.y < discretized_position.y){
            transform.translation = discretized_position;
            fall.state = FallState::Stopped;
            if maybe_controllable.is_some() {
                next_piece_event.send_default();
            }

            grid.place_cell(*position, Some(*piece));
        }
    } 
}

