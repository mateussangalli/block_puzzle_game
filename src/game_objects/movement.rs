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

#[derive(Component)]
pub struct Collider;

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub offset: Vec3,
    pub collision: Collision
}

impl CollisionEvent {
    pub fn new(entity: Entity, offset: Vec3, collision: Collision) -> Self {
        Self {entity, offset, collision}
    }
}

pub fn update_fall(mut query: Query<(&mut Transform, &Fall)>, time: Res<Time>) {
    for (mut transform, fall) in query.iter_mut() {
        match &fall.state {
            FallState::Stopped => (),
            FallState::Fast => transform.translation.y -= FAST_MULT * fall.velocity * time.delta_seconds(),
            FallState::Normal => transform.translation.y -= fall.velocity * time.delta_seconds(),
        }
    }
}

