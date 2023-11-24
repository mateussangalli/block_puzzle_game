use bevy::{
    math::vec3,
    prelude::{Component, Mut, Transform, Vec2, Vec3},
};
use std::{ops::{Index, IndexMut}, fmt::Display};

#[derive(Component)]
pub struct Grid<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
    pub cell_size: f32,
    left_bottom_corner: Vec2,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

impl GridPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn translate(&self, row: usize, col: usize) -> Self {
        Self {
            row: self.row + row,
            col: self.col + col,
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
        let x = self.left_bottom_corner.x + self.cell_size * (grid_position.col as f32);
        let y = self.left_bottom_corner.y + self.cell_size * (grid_position.row as f32);
        vec3(x, y, 0.)
    }

    pub fn vec3_to_position(&self, vector: Vec3) -> GridPosition {
        let col = ((vector.x - self.left_bottom_corner.x) / self.cell_size).round() as usize;
        let row = ((vector.y - self.left_bottom_corner.y) / self.cell_size).round() as usize;
        GridPosition::new(row, col)
    }

    pub fn place_cell(&mut self, grid_position: GridPosition, value: T) {
        self[grid_position.row][grid_position.col] = value;
    }
}

impl<T> Grid<Option<T>> {
    pub fn left_is_empty(&self, grid_position: GridPosition) -> bool {
        grid_position.col > 0 && self[grid_position.row][grid_position.col - 1].is_none()
    }

    pub fn right_is_empty(&self, grid_position: GridPosition) -> bool {
        (grid_position.col < self.width - 1)
            && self[grid_position.row][grid_position.col + 1].is_none()
    }

    pub fn below_is_empty(&self, grid_position: GridPosition) -> bool {
        (grid_position.row > 0)
            && self[grid_position.row - 1][grid_position.col].is_none()
    }

    pub fn move_right(&self, mut transform: Mut<Transform>, mut grid_position: Mut<GridPosition>) {
        if self.right_is_empty(*grid_position) {
            transform.translation.x += self.cell_size;
            grid_position.col += 1;
        }
    }

    pub fn move_left(&self, mut transform: Mut<Transform>, mut grid_position: Mut<GridPosition>) {
        if self.left_is_empty(*grid_position) {
            transform.translation.x -= self.cell_size;
            grid_position.col -= 1;
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &[T] {
        &self.data[self.width * index..self.width * (index + 1)]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[self.width * index..self.width * (index + 1)]
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::vec2;

    use super::*;

    #[test]
    fn test_index() {
        let grid = Grid::new(2, 2, vec![1, 2, 3, 4], 1., vec2(1., 1.));

        assert!(grid[0][1] == 2);
        assert!(grid[1][0] == 3);
    }

    #[test]
    fn test_move_right() {
        let grid: Grid<Option<u8>> = Grid::new(2, 2, vec![None; 4], 1., vec2(1., 1.));
        let grid_position1 = GridPosition::new(0, 0);
        let grid_position2 = GridPosition::new(0, 1);

        assert!(grid.right_is_empty(grid_position1));
        assert!(!grid.right_is_empty(grid_position2));
    }
}
