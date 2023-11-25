//! Renders a 2D scene containing a single, moving sprite.
mod game_objects;

use bevy::{math::vec3, prelude::*};

use crate::game_objects::{
    fall::{update_fall_pair, update_fall_piece},
    movement::{rotate_pair, move_pair},
    piece::{spawn_next_piece, PairLandedEvent, setup, spawn_piece},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<PairLandedEvent>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_piece)
        .add_systems(
            FixedUpdate,
            (
                move_pair,
                rotate_pair,
                update_fall_pair,
                update_fall_piece,
                spawn_next_piece,
            )
                .chain(),
        )
        .run();
}
