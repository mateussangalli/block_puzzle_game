use bevy::{
    math::vec3,
    prelude::{Component, Mut, Transform, Vec2, Vec3},
};
use std::{ops::{Index, IndexMut}, task::Wake};

use crate::game_objects::piece::{Pair, PairOrientation, Piece};

#[derive(Component)]
pub struct Grid<T> {
    pub height: usize,
    pub width: usize,
    data: Vec<T>,
    pub cell_size: f32,
    pub left_bottom_corner: Vec2,
}

#[derive(Component, Clone, Copy, Debug)]
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

pub type GameGrid = Grid<Option<Piece>>;

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
}
