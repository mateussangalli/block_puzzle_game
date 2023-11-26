use bevy::{
    math::vec3,
    prelude::{Component, Entity, Mut, Transform, Vec2, Vec3},
    utils::HashSet,
};
use std::{
    ops::{Index, IndexMut},
    task::Wake,
};

use crate::game_objects::piece::{Pair, PairOrientation, Piece, PieceColor};

#[derive(Component)]
pub struct Grid<T> {
    pub height: usize,
    pub width: usize,
    data: Vec<T>,
    pub cell_size: f32,
    pub left_bottom_corner: Vec2,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition {
    value: [isize; 2],
}

impl GridPosition {
    pub fn new(row: isize, col: isize) -> Self {
        Self { value: [row, col] }
    }

    pub fn row(self) -> isize {
        self.value[0]
    }

    pub fn col(self) -> isize {
        self.value[1]
    }

    pub fn translate(&self, row: isize, col: isize) -> Self {
        Self {
            value: [self.row() + row, self.col() + col],
        }
    }
}

impl<T> Grid<T> {
    pub fn new(
        height: usize,
        width: usize,
        data: Vec<T>,
        cell_size: f32,
        left_bottom_corner: Vec2,
    ) -> Self {
        assert!(width * height == data.len());
        Self {
            height,
            width,
            data,
            cell_size,
            left_bottom_corner,
        }
    }

    pub fn position_to_vec3(&self, grid_position: GridPosition) -> Vec3 {
        let x = self.left_bottom_corner.x + self.cell_size * (grid_position.col() as f32);
        let y = self.left_bottom_corner.y + self.cell_size * (grid_position.row() as f32);
        vec3(x, y, 0.)
    }

    pub fn vec3_to_position(&self, vector: Vec3) -> GridPosition {
        let col = ((vector.x - self.left_bottom_corner.x) / self.cell_size).round() as isize;
        let row = ((vector.y - self.left_bottom_corner.y) / self.cell_size).round() as isize;
        GridPosition::new(row, col)
    }

    pub fn place_cell(&mut self, grid_position: GridPosition, value: T) {
        self[grid_position.value] = value;
    }
}

impl<T> Grid<Option<T>> {
    pub fn is_valid(&self, grid_position: GridPosition) -> bool {
        grid_position.row() < self.height as isize
            && grid_position.col() < self.width as isize
            && grid_position.row() >= 0
            && grid_position.col() >= 0
    }

    pub fn is_empty(&self, grid_position: GridPosition) -> bool {
        self.is_valid(grid_position) && (self[grid_position.value]).is_none()
    }

    pub fn can_move_left(&self, grid_position: GridPosition) -> bool {
        self.is_empty(grid_position.translate(0, -1))
    }

    pub fn can_move_right(&self, grid_position: GridPosition) -> bool {
        self.is_empty(grid_position.translate(0, 1))
    }

    pub fn can_move_down(&self, grid_position: GridPosition) -> bool {
        self.is_empty(grid_position.translate(-1, 0))
    }

    pub fn move_right_pair(
        &self,
        pair: Pair,
        transform: &mut Transform,
        grid_position: &mut GridPosition,
    ) {
        if self.can_move_right(*grid_position)
            && self.can_move_right(pair.get_second_position(*grid_position))
        {
            transform.translation.x += self.cell_size;
            grid_position.value[1] += 1;
        }
    }

    pub fn move_left_pair(
        &self,
        pair: Pair,
        transform: &mut Transform,
        grid_position: &mut GridPosition,
    ) {
        if self.can_move_left(*grid_position)
            && self.can_move_left(pair.get_second_position(*grid_position))
        {
            transform.translation.x -= self.cell_size;
            grid_position.value[1] -= 1;
        }
    }

    pub fn can_turn_clockwise(&self, pair: Pair, grid_position: GridPosition) -> bool {
        let new_position = pair.turn_clockwise().get_second_position(grid_position);
        self.is_empty(new_position)
    }
}

impl<T> Index<[isize; 2]> for Grid<T> {
    type Output = T;
    fn index(&self, index: [isize; 2]) -> &T {
        let row = index[0] as usize;
        let col = index[1] as usize;
        &self.data[self.width * row + col]
    }
}

impl<T> IndexMut<[isize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [isize; 2]) -> &mut T {
        let row = index[0] as usize;
        let col = index[1] as usize;
        &mut self.data[self.width * row + col]
    }
}

impl<T> Index<GridPosition> for Grid<T> {
    type Output = T;
    fn index(&self, index: GridPosition) -> &T {
        &self[index.value]
    }
}

impl<T> IndexMut<GridPosition> for Grid<T> {
    fn index_mut(&mut self, index: GridPosition) -> &mut T {
        &mut self[index.value]
    }
}

fn get_adjacent(position: GridPosition) -> Vec<GridPosition> {
    vec![
        position.translate(1, 0),
        position.translate(-1, 0),
        position.translate(0, 1),
        position.translate(0, -1),
    ]
}

fn add_position(
    position: GridPosition,
    conn_comp: &mut Vec<GridPosition>,
    adjacent: &mut Vec<GridPosition>,
    seen: &mut HashSet<GridPosition>,
) {
    conn_comp.push(position);
    for p in get_adjacent(position).into_iter() {
        if !seen.contains(&p) {
            seen.insert(p);
            adjacent.push(p);
        }
    }
}

impl<T> Grid<Option<(PieceColor, T)>> {
    pub fn find_conn_comp(&self, initial_position: GridPosition) -> Vec<GridPosition> {
        let initial_color;
        match self[initial_position] {
            None => return vec![],
            Some((color, _)) => initial_color = color,
        }

        let mut conn_comp: Vec<GridPosition> = vec![initial_position];

        let mut adjacent = get_adjacent(initial_position);
        let mut seen = HashSet::from_iter(adjacent.clone().into_iter());
        seen.insert(initial_position);

        while let Some(position) = adjacent.pop() {
            if !self.is_valid(position) { continue }
            if let Some((color, _)) = self[position] {
                if color == initial_color {
                    add_position(position, &mut conn_comp, &mut adjacent, &mut seen);
                }
            }
        }

        conn_comp
    }
}

pub type GameGrid = Grid<Option<(PieceColor, Entity)>>;

#[cfg(test)]
mod tests {
    use bevy::math::vec2;

    use super::*;

    #[test]
    fn test_index() {
        let grid = Grid::new(2, 2, vec![1, 2, 3, 4], 1., vec2(1., 1.));

        assert!(grid[[0, 1]] == 2);
        assert!(grid[[1, 0]] == 3);
    }

    #[test]
    fn test_move_right() {
        let grid: Grid<Option<u8>> = Grid::new(2, 2, vec![None; 4], 1., vec2(1., 1.));
        let grid_position1 = GridPosition::new(0, 0);
        let grid_position2 = GridPosition::new(0, 1);

        assert!(grid.can_move_right(grid_position1));
        assert!(!grid.can_move_right(grid_position2));
    }

    #[derive(Clone, Copy)]
    struct Dummy;

    #[test]
    fn test_conn_comp() {
        let mut grid: Grid<Option<(PieceColor, Dummy)>> = Grid::new(2, 2, vec![None; 4], 1., vec2(1., 1.));
        grid[[0, 0]] = Some((PieceColor::Red, Dummy));
        grid[[0, 1]] = Some((PieceColor::Red, Dummy));
        grid[[1, 0]] = Some((PieceColor::Blue, Dummy));
        let initial_position = GridPosition::new(0, 0);

        let expected = vec![GridPosition::new(0, 0), GridPosition::new(0, 1)];

        let output = grid.find_conn_comp(initial_position);

        assert!(expected == output)

    }
}
