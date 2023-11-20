//! Renders a 2D scene containing a single, moving sprite.
mod game_objects;

use bevy::{prelude::*, math::vec3};
use game_objects::piece::Bag;
use game_objects::movement::update_fall;
use game_objects::wall::{WallBundle, WallLocation};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, generate_next_piece)
        .add_systems(Update, update_fall)
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

fn generate_next_piece(mut commands: Commands, mut query: Query<&mut Bag>) {
    let mut bag = query.single_mut();
    commands.spawn(bag.new_piece(vec3(0., 0., 0.)));
}
