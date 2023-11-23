use std::ops::{Index, IndexMut};
use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Grid<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
    cell_size: f32,
    left_bottom_corner: Vec2
}

impl<T> Grid<T> {
    pub fn new(height: usize, width: usize, data: Vec<T>, cell_size: f32, left_bottom_corner: Vec2) -> Self {
        assert!(width * height == data.len());
        Self { height, width, data, cell_size, left_bottom_corner }
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
