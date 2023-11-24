use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::game_objects::{
    fall::Fall,
    grid::{Grid, GridPosition},
    movement::InputTimer
};

const REPEAT_DELAY: f32 = 0.03;
const START_DELAY: f32 = 0.1;

const PIECE_SIZE: f32 = 32.;
const PIECE_FALL_SPEED: f32 = 150.;
const PIECE_LATERAL_SPEED: f32 = 400.;

const STARTING_ROW: usize = 20;
const STARTING_COL: usize = 5;
const GRID_HEIGHT: usize = 19;
const GRID_WIDTH: usize = 10;
const LEFT_BOTTOM_CORNER: Vec2 = vec2(-200., -400.);

const RED: Color = Color::rgb(1., 0., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const PURPLE: Color = Color::rgb(0.5, 0., 0.5);

#[derive(Component, Clone, Copy, Debug)]
pub enum PieceColor {
    Red,
    Blue,
    Purple,
    Green,
}

impl PieceColor {
    fn get_color(self) -> Color {
        match self {
            PieceColor::Red => RED,
            PieceColor::Blue => BLUE,
            PieceColor::Green => GREEN,
            PieceColor::Purple => PURPLE,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
}

#[derive(Component)]
pub struct Controllable;

#[derive(Event, Default)]
pub struct NextPieceEvent;

#[derive(Bundle)]
pub struct PieceBundle {
    color: PieceColor,
    sprite_bundle: SpriteBundle,
    fall: Fall,
    piece: Piece,
    grid_position: GridPosition,
}

#[derive(Component)]
pub struct Bag {
    rng: StdRng,
}

impl Bag {
    pub fn new() -> Self {
        let rng = StdRng::from_entropy();
        Self { rng }
    }

    fn piece_color(&mut self) -> PieceColor {
        let r = self.rng.next_u32() % 4;

        match r {
            0 => PieceColor::Red,
            1 => PieceColor::Blue,
            2 => PieceColor::Purple,
            3 => PieceColor::Green,
            _ => panic!(),
        }
    }

    pub fn new_piece(&mut self, grid_position: GridPosition, position: Vec3) -> PieceBundle {
        let color = self.piece_color();
        PieceBundle {
            color,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: color.get_color(),
                    ..default()
                },
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(PIECE_SIZE, PIECE_SIZE, 1.0),
                    ..default()
                },
                ..default()
            },
            fall: Fall::new(PIECE_FALL_SPEED),
            piece: Piece { color },
            grid_position: grid_position,
        }
    }
}

pub type GameGrid = Grid<Option<Piece>>;

pub fn spawn_piece(
    mut commands: Commands,
    mut query: Query<(&mut Bag, &GameGrid)>,
) {
    let (mut bag, grid) = query.single_mut();
    let grid_position = GridPosition::new(STARTING_ROW, STARTING_COL);

    println!("Spawning piece");
    commands.spawn((
        bag.new_piece(grid_position, grid.position_to_vec3(grid_position)),
        Controllable,
    ));
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let bag = Bag::new();

    let grid = GameGrid::new(
        GRID_HEIGHT,
        GRID_WIDTH,
        vec![None; GRID_WIDTH * GRID_HEIGHT],
        PIECE_SIZE,
        LEFT_BOTTOM_CORNER,
    );

    let input_timer = InputTimer::new(REPEAT_DELAY, START_DELAY);

    commands.spawn((bag, grid, input_timer));
}
