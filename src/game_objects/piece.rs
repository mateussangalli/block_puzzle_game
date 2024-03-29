use std::{task::Wake, cmp::min};

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::game_objects::{
    fall::{Fall, FallState},
    grid::{GameGrid, GridPosition},
    movement::DASTimer,
};

const REPEAT_DELAY: f32 = 0.03;
const START_DELAY: f32 = 0.1;

const PIECE_SIZE: f32 = 32.;
const PIECE_FALL_SPEED: f32 = 150.;

const STARTING_ROW: isize = 18;
const STARTING_COL: isize = 5;
const GRID_HEIGHT: usize = 20;
const GRID_WIDTH: usize = 10;
const LEFT_BOTTOM_CORNER: Vec2 = vec2(-200., -300.);

const MIN_SIZE_SCORE: usize = 4;

const RED: Color = Color::rgb(1., 0., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const PURPLE: Color = Color::rgb(0.5, 0., 0.5);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
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
pub struct PairLandedEvent;

#[derive(Event)]
pub struct PieceLandedEvent {
    pub entity: Entity,
}

impl PieceLandedEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Bundle)]
pub struct PieceBundle {
    color: PieceColor,
    sprite_bundle: SpriteBundle,
    piece: Piece,
}

impl PieceBundle {
    pub fn new(color: PieceColor, position: Vec3) -> Self {
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
            piece: Piece { color },
        }
    }
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

    pub fn new_piece(&mut self, position: Vec3) -> PieceBundle {
        let color = self.piece_color();
        PieceBundle::new(color, position)
    }
}

#[derive(Clone, Copy)]
pub enum PairOrientation {
    ABVertical,
    ABHorizontal,
    BAVertical,
    BAHorizontal,
}

#[derive(Component, Clone, Copy)]
pub struct Pair {
    orientation: PairOrientation,
}

impl Pair {
    pub fn new() -> Self {
        Pair {
            orientation: PairOrientation::ABVertical,
        }
    }

    pub fn get_second_position(self, position: GridPosition) -> GridPosition {
        match self.orientation {
            PairOrientation::ABVertical => position.translate(-1, 0),
            PairOrientation::BAVertical => position.translate(1, 0),
            PairOrientation::ABHorizontal => position.translate(0, 1),
            PairOrientation::BAHorizontal => position.translate(0, -1),
        }
    }

    pub fn turn_clockwise(self) -> Self {
        let new_orientation = match self.orientation {
            PairOrientation::ABVertical => PairOrientation::BAHorizontal,
            PairOrientation::BAHorizontal => PairOrientation::BAVertical,
            PairOrientation::BAVertical => PairOrientation::ABHorizontal,
            PairOrientation::ABHorizontal => PairOrientation::ABVertical,
        };

        Pair {
            orientation: new_orientation,
        }
    }

    pub fn adjust_transform(self, transform: &mut Transform, order: PieceOrder) {
        match (self.orientation, order) {
            (_, PieceOrder::First) => (),
            (PairOrientation::ABVertical, _) => transform.translation = vec3(0., -PIECE_SIZE, 0.),
            (PairOrientation::BAVertical, _) => transform.translation = vec3(0., PIECE_SIZE, 0.),
            (PairOrientation::ABHorizontal, _) => transform.translation = vec3(PIECE_SIZE, 0., 0.),
            (PairOrientation::BAHorizontal, _) => transform.translation = vec3(-PIECE_SIZE, 0., 0.),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum PieceOrder {
    First,
    Second,
}

pub fn spawn_piece(mut commands: Commands, mut query: Query<(&mut Bag, &GameGrid)>) {
    let (mut bag, grid) = query.single_mut();
    let grid_position = GridPosition::new(STARTING_ROW, STARTING_COL);

    println!("Spawning piece");
    commands
        .spawn((
            VisibilityBundle {
                visibility: Visibility::Visible,
                ..default()
            },
            Pair::new(),
            Fall::new(PIECE_FALL_SPEED),
            Transform::from_translation(grid.position_to_vec3(grid_position)),
            GlobalTransform::IDENTITY,
            grid_position,
        ))
        .with_children(|parent| {
            parent.spawn((PieceOrder::First, bag.new_piece(vec3(0., 0., 0.))));
            parent.spawn((PieceOrder::Second, bag.new_piece(vec3(0., -PIECE_SIZE, 0.))));
        });
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

    let input_timer = DASTimer::new(REPEAT_DELAY, START_DELAY);

    let grid_middle = LEFT_BOTTOM_CORNER
        + PIECE_SIZE * 0.5 * vec2((GRID_WIDTH - 1) as f32, (GRID_HEIGHT - 1) as f32);
    let grid_middle = vec3(grid_middle.x, grid_middle.y, -1.);
    let grid_size = vec3(
        PIECE_SIZE * GRID_WIDTH as f32,
        PIECE_SIZE * GRID_HEIGHT as f32,
        1.,
    );
    let grid_background = SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            ..default()
        },
        transform: Transform {
            translation: grid_middle,
            scale: grid_size,
            ..default()
        },
        ..default()
    };

    commands.spawn((bag, grid, input_timer, grid_background));
}

pub fn spawn_next_piece(
    mut commands: Commands,
    mut land_event: EventReader<PairLandedEvent>,
    query_entity: Query<(Entity, &GridPosition, &Pair, &Children)>,
    query_children: Query<(Entity, &Piece, &PieceOrder)>,
    mut query_bag: Query<(&mut Bag, &GameGrid)>,
) {
    let mut flag = false;
    for event in land_event.read() {
        flag = true;
    }

    if flag {
        let (mut bag, grid) = query_bag.single_mut();
        let (entity_pair, position1, pair, children) = query_entity.single();
        let position2 = pair.get_second_position(*position1);

        // despawn children and spawn them with commands
        for &child in children.iter() {
            match query_children.get(child) {
                Ok((entity, piece, PieceOrder::First)) => {
                    commands.spawn((
                        PieceBundle::new(piece.color, grid.position_to_vec3(*position1)),
                        *position1,
                        Fall::new(PIECE_FALL_SPEED),
                    ));
                    commands.get_entity(entity).unwrap().despawn();
                }
                Ok((entity, piece, PieceOrder::Second)) => {
                    commands.spawn((
                        PieceBundle::new(piece.color, grid.position_to_vec3(position2)),
                        position2,
                        Fall::new(PIECE_FALL_SPEED),
                    ));
                    commands.get_entity(entity).unwrap().despawn();
                }
                _ => (),
            }
        }

        commands.entity(entity_pair).remove::<Pair>();

        // spawn new piece
        let starting_position = GridPosition::new(STARTING_ROW, STARTING_COL);
        commands
            .spawn((
                VisibilityBundle {
                    visibility: Visibility::Visible,
                    ..default()
                },
                Pair::new(),
                Fall::new(PIECE_FALL_SPEED),
                Transform::from_translation(grid.position_to_vec3(starting_position)),
                GlobalTransform::IDENTITY,
                starting_position,
            ))
            .with_children(|parent| {
                parent.spawn((PieceOrder::First, bag.new_piece(vec3(0., 0., 0.))));
                parent.spawn((PieceOrder::Second, bag.new_piece(vec3(0., -PIECE_SIZE, 0.))));
            });
    }
}

pub fn check_connected(
    mut commands: Commands,
    mut query_grid: Query<&mut GameGrid>,
    query_position: Query<&GridPosition>,
    mut query_fall: Query<&mut Fall>,
    mut land_event: EventReader<PieceLandedEvent>,
){
    let mut grid = query_grid.single_mut();
    let mut conn_comps: Vec<Vec<GridPosition>> = Vec::with_capacity(land_event.len());

    for event in land_event.read() {
        if let Ok(grid_position) = query_position.get(event.entity) {
            conn_comps.push(grid.find_conn_comp(*grid_position));
        }
    }

    let mut min_heights = vec![grid.height as isize; grid.width];

    for conn_comp in conn_comps {
        if conn_comp.len() < MIN_SIZE_SCORE { continue }

        for position in conn_comp {
            if let Some((_, entity)) = grid[position] {
                commands.entity(entity).despawn();
            }

            grid[position] = None;

            min_heights[position.col() as usize] = min(position.row(), min_heights[position.col() as usize]);
        }
    }


    for col in 0..grid.width {
        for row in min_heights[col as usize]..(grid.height as isize) {
            let position = GridPosition::new(row, col as isize);

            if let Some((_, entity)) = grid[position] {
                grid[position] = None;

                if let Ok(mut fall) = query_fall.get_mut(entity) {
                    fall.state = FallState::Normal;
                    // println!("{row}, {col}")
                }
            }
        }
    }
}
