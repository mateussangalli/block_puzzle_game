//! Renders a 2D scene containing a single, moving sprite.
mod game_objects;

use bevy::{math::vec3, prelude::*};

use game_objects::{
    movement::{update_fall, CollisionEvent},
    piece::{check_for_collisions, spawn_next_piece, Bag, move_active_piece, NextPieceEvent},
    wall::{WallBundle, WallLocation},
};

use crate::game_objects::piece::Controllable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_first_piece)
        // .add_event::<CollisionEvent>()
        .add_event::<NextPieceEvent>()
        .add_systems(
            FixedUpdate,
            (
                move_active_piece,
                update_fall,
                check_for_collisions,
                spawn_next_piece,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Collider;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Bag::new());

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}

fn spawn_first_piece(mut commands: Commands, mut query: Query<&mut Bag>) {
    let mut bag = query.single_mut();
    commands.spawn((bag.new_piece(vec3(0., 200., 0.)), Controllable::new()));
}
