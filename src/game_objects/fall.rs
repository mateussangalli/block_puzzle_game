use std::task::Wake;

use bevy::sprite::collide_aabb::collide;
use bevy::{math::vec2, prelude::*, sprite::collide_aabb::Collision};

use crate::game_objects::{
    grid::{GameGrid, GridPosition},
    piece::{Controllable, Pair, PairLandedEvent, Piece},
};

const FAST_MULT: f32 = 3.;

#[derive(Component)]
pub struct Fall {
    velocity: f32,
    pub state: FallState,
}

#[derive(Clone, Copy)]
pub enum FallState {
    Fast,
    Normal,
    Stopped,
}

impl Fall {
    pub fn new(velocity: f32) -> Self {
        Self {
            velocity,
            state: FallState::Normal,
        }
    }

    pub fn get_velocity(&self) -> f32 {
        match self.state {
            FallState::Fast => self.velocity * FAST_MULT,
            FallState::Normal => self.velocity,
            FallState::Stopped => 0.,
        }
    }
}

pub fn update_fall_piece(
    mut query_piece: Query<(
        Entity,
        &mut Transform,
        &mut GridPosition,
        &mut Fall,
        &Piece,
    )>,
    mut query_grid: Query<&mut GameGrid>,
    time: Res<Time>,
) {
    let mut grid = query_grid.single_mut();
    
    for (entity, mut transform, mut position, mut fall, piece) in query_piece.iter_mut() {
        transform.translation.y -= time.delta_seconds() * fall.get_velocity();
        *position = grid.vec3_to_position(transform.translation);
        let discretized_position = grid.position_to_vec3(*position);
        if !grid.can_move_down(*position) && (transform.translation.y < discretized_position.y){
            transform.translation = discretized_position;
            fall.state = FallState::Stopped;
    
            grid.place_cell(*position, Some(entity));
        }
    }
}

pub fn update_fall_pair(
    mut query_pair: Query<(&mut Transform, &mut GridPosition, &mut Fall, &Pair)>,
    mut query_grid: Query<&mut GameGrid>,
    time: Res<Time>,
    mut land_event: EventWriter<PairLandedEvent>,
) {
    let mut grid = query_grid.single_mut();

    let (mut transform, mut position, mut fall, pair) = query_pair.single_mut();

    transform.translation.y -= time.delta_seconds() * fall.get_velocity();
    *position = grid.vec3_to_position(transform.translation);

    let discretized_position = grid.position_to_vec3(*position);
    let can_move_down_pair =
        grid.can_move_down(*position) && grid.can_move_down(pair.get_second_position(*position));
    if !can_move_down_pair && (transform.translation.y < discretized_position.y) {
        transform.translation = discretized_position;
        fall.state = FallState::Stopped;

        land_event.send_default();
    }
}
