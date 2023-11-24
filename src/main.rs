//! Renders a 2D scene containing a single, moving sprite.
mod game_objects;

use bevy::{math::vec3, prelude::*};

use game_objects::piece::{spawn_piece, setup};
use game_objects::movement::{move_active};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_piece)
        .add_systems(Update, move_active)
        .run();
}

