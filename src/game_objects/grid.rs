use std::ops::{Index, IndexMut};
use bevy::{prelude::{Component, Vec2, Vec3}, math::vec3};

#[derive(Component)]
pub struct Grid<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
    cell_size: f32,
    left_bottom_corner: Vec2
}

#[derive(Component, Clone, Copy)]
pub struct GridPosition {
    row: usize,
    col: usize
}

impl GridPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self {row, col}
    }

    pub fn translate(&self, row: usize, col: usize) -> Self {
        Self {row: self.row + row, col: self.col + col}
    }
}

impl<T> Grid<T> {
    pub fn new(height: usize, width: usize, data: Vec<T>, cell_size: f32, left_bottom_corner: Vec2) -> Self {
        assert!(width * height == data.len());
        Self { height, width, data, cell_size, left_bottom_corner }
    }

    pub fn position_to_vec3(&self, grid_position: GridPosition) -> Vec3 {
        let x = self.left_bottom_corner.x + self.cell_size * (grid_position.col as f32);
        let y = self.left_bottom_corner.y + self.cell_size * (grid_position.row as f32);
        vec3(x, y, 0.)
    }
}

impl<T> Grid<Option<T>> {
    pub fn left_is_empty(&self, grid_position: GridPosition) -> bool {
        self[grid_position.row][grid_position.col - 1].is_none()
    }

    pub fn right_is_empty(&self, grid_position: GridPosition) -> bool {
        self[grid_position.row][grid_position.col + 1].is_none()
    }
}


impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &[T] {
        &self.data[self.width * index .. self.width * (index+1)]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[self.width * index .. self.width * (index+1)]
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
}
