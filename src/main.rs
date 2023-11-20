//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use rand::{prelude::StdRng, RngCore, SeedableRng};

// TODO: implement falling
// TODO: implement board structure

const PIECE_SIZE: f32 = 32.;
const RED: Color = Color::rgb(1., 0., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const PURPLE: Color = Color::rgb(0.5, 0., 0.5);
const DOWNWARD_SPEED: f32 = 100.;
const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const WALL_THICKNESS: f32 = 10.;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, generate_next_piece)
        .add_systems(Update, update_falling_pair)
        .run();
}

#[derive(Component, Clone, Copy)]
enum Piece {
    Red,
    Blue,
    Purple,
    Green,
}

impl Piece {
    fn get_color(self) -> Color {
        match self {
            Piece::Red => RED,
            Piece::Blue => BLUE,
            Piece::Green => GREEN,
            Piece::Purple => PURPLE,
        }
    }
}

#[derive(Component)]
struct FallingPair;

#[derive(Component)]
struct Bag {
    rng: StdRng,
}

#[derive(Component)]
struct Collider;

impl Bag {
    fn new() -> Self {
        let rng = StdRng::from_entropy();
        Self { rng }
    }

    fn next_piece(&mut self) -> Piece {
        let r = self.rng.next_u32() % 4;

        match r {
            0 => Piece::Red,
            1 => Piece::Blue,
            2 => Piece::Purple,
            3 => Piece::Green,
            _ => panic!(),
        }
    }

    fn spawn_piece(&mut self, builder: &mut ChildBuilder, position: Vec3) {
        let piece = self.next_piece();
        builder.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: piece.get_color(),
                    ..default()
                },
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                    ..default()
                },
                ..default()
            },
            piece,
            Collider
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Bag::new());

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}

fn generate_next_piece(mut commands: Commands, mut query: Query<&mut Bag>) {
    dbg!(&query);
    let mut bag = query.single_mut();
    println!("spawn");
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::IDENTITY,
                ..default()
            },
            FallingPair,
        ))
        .with_children(|parent| {
            bag.spawn_piece(parent, Vec3::new(-PIECE_SIZE, 0., 0.));
            bag.spawn_piece(parent, Vec3::new(0., 0., 0.));
        });
    
}

fn update_falling_pair(mut query: Query<(&mut Transform, &FallingPair)>, time: Res<Time>) {
    if let Some((mut transform, falling_pair)) = query.iter_mut().next() {
        transform.translation.y -= DOWNWARD_SPEED * time.delta_seconds();
    }

} 


// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

