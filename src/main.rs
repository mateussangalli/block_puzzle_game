//! Renders a 2D scene containing a single, moving sprite.
mod game_objects;

use bevy::{math::vec3, prelude::*};

use game_objects::piece::{spawn_piece, setup};
use game_objects::movement::{move_active};
use game_objects::fall::{update_fall};

use crate::game_objects::piece::{NextPieceEvent, spawn_next_piece};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<NextPieceEvent>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_piece)
        .add_systems(FixedUpdate, (move_active, update_fall, spawn_next_piece).chain())
        .run();
}

