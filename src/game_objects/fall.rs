use std::task::Wake;

use bevy::{prelude::*, math::vec2, sprite::collide_aabb::Collision};
use bevy::sprite::collide_aabb::collide;

const FAST_MULT: f32 = 3.;

#[derive(Component)]
pub struct Fall {
    velocity: f32,
    pub state: FallState
}

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
}
