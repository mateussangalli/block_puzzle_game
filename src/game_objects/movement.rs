use std::task::Wake;

use bevy::{prelude::*, math::vec2, sprite::collide_aabb::Collision};
use bevy::sprite::collide_aabb::collide;

#[derive(Component)]
pub struct Fall {
    velocity: f32,
    frozen: bool,
}

impl Fall {
    pub fn new(velocity: f32) -> Self {
        Self {
            velocity,
            frozen: false,
        }
    }

    pub fn stop(&mut self) {
        self.frozen = true;
    }

    pub fn resume(&mut self) {
        self.frozen = false;
    }
}

#[derive(Component)]
pub struct Collider;

pub fn update_fall(mut query: Query<(&mut Transform, &Fall)>, time: Res<Time>) {
    for (mut transform, fall) in query.iter_mut() {
        if !fall.frozen {
            transform.translation.y -= fall.velocity * time.delta_seconds();
        }
    }
}

