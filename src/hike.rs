use std::collections::{HashSet, VecDeque};

use simple_grid::{Grid, GridIndex};

pub struct TopographyMap {
    grid: Grid<Cell>,
}

impl TopographyMap {
    pub fn new(grid: Grid<u8>) -> Self {
        let cell_grid = Grid::new(
            grid.width(),
            grid.height(),
            grid.cell_iter().map(|&c| Cell { height: c }).collect(),
        );
        Self { grid: cell_grid }
    }

    pub fn trailheads(&self) -> impl Iterator<Item = GridIndex> + use<'_> {
        self.grid.cells_with_indices_iter().filter_map(|(i, c)| {
            if c.is_trailhead() {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn score(&self, trailhead: GridIndex) -> Option<u32> {
        let Some(cell) = self.grid.get(trailhead) else {
            return None;
        };
        if !cell.is_trailhead() {
            return None;
        }

        let mut to_visit = VecDeque::with_capacity(self.grid.area());
        to_visit.push_back(trailhead);

        let mut visited = HashSet::new();
        let mut score = 0;
        while let Some(current) = to_visit.pop_front() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            let current_cell = self.grid[current];
            if current_cell.is_peak() {
                score += 1;
                continue;
            }

            for neighbor_idx in self.grid.cardinal_neighbor_indices_of(current) {
                let neighbor = self.grid[neighbor_idx];
                if !visited.contains(&neighbor_idx) && neighbor.is_reachable_from(current_cell) {
                    to_visit.push_back(neighbor_idx);
                }
            }
        }

        Some(score)
    }

    pub fn rating(&self, trailhead: GridIndex) -> Option<u32> {
        let Some(cell) = self.grid.get(trailhead) else {
            return None;
        };
        if !cell.is_trailhead() {
            return None;
        }

        let mut to_visit = VecDeque::with_capacity(self.grid.area());
        to_visit.push_back(trailhead);

        let mut rating = 0;
        while let Some(current) = to_visit.pop_front() {
            let current_cell = self.grid[current];
            if current_cell.is_peak() {
                rating += 1;
                continue;
            }

            for neighbor_idx in self.grid.cardinal_neighbor_indices_of(current) {
                let neighbor = self.grid[neighbor_idx];
                if neighbor.is_reachable_from(current_cell) {
                    to_visit.push_back(neighbor_idx);
                }
            }
        }

        Some(rating)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Cell {
    height: u8,
}

impl Cell {
    pub fn is_trailhead(&self) -> bool {
        self.height == 0
    }

    pub fn is_peak(&self) -> bool {
        self.height == 9
    }

    pub fn is_reachable_from(&self, from: Self) -> bool {
        from.height < self.height && self.height - from.height == 1
    }
}
