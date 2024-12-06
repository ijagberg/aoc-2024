use simple_grid::{Grid, GridIndex};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Index,
};

pub struct GuardMap {
    grid: Grid<Cell>,
    guard_start: (GridIndex, Direction),
}

impl GuardMap {
    pub fn new(mut grid: Grid<Cell>) -> Result<Self, &'static str> {
        let mut guard_idx: Vec<GridIndex> = grid
            .indices()
            .filter(|i| matches!(grid[*i], Cell::Guard(_)))
            .collect();
        if guard_idx.len() != 1 {
            return Err("invalid number of guards");
        } else {
            let guard_idx = guard_idx.remove(0);
            let &guard_direction = grid[guard_idx].as_guard().unwrap();
            grid[guard_idx] = Cell::Empty;
            Ok(Self {
                grid,
                guard_start: (guard_idx, guard_direction),
            })
        }
    }

    pub fn get_guard_walk(&self) -> Result<Vec<(GridIndex, Direction)>, &'static str> {
        self.guard_walk_from(self.guard_start)
    }

    fn guard_walk_from(
        &self,
        start: (GridIndex, Direction),
    ) -> Result<Vec<(GridIndex, Direction)>, &'static str> {
        let mut walk = Vec::with_capacity(self.grid.area());

        let mut visited = HashSet::with_capacity(self.grid.area());
        let mut current = start;
        loop {
            let (index, direction) = current;
            walk.push(current);
            if visited.contains(&current) {
                return Err("walk contains loop");
            }
            visited.insert(current);

            if let Some(next) = self.next_guard(index, direction) {
                current = next;
            } else {
                break;
            }
        }

        Ok(walk)
    }

    pub fn get_obstacle_places(mut self) -> HashSet<GridIndex> {
        let mut obstacle_places = HashSet::new();
        let original_walk = self.get_guard_walk().unwrap();
        for (index, direction) in original_walk {
            if let Some(next_index) = Self::next_index(direction, index) {
                if next_index == self.guard_start.0 {
                    // cant place an obstacle at the guards starting position
                    continue;
                }
                if let Some(Cell::Empty) = self.grid.get(next_index) {
                    // the guard would continue forward, so try putting an obstacle there
                    self.grid[next_index] = Cell::Wall;
                    if let Err(_) = self.get_guard_walk() {
                        obstacle_places.insert(next_index);
                    }
                    self.grid[next_index] = Cell::Empty;
                }
            }
        }

        obstacle_places
    }

    fn next_guard(&self, index: GridIndex, direction: Direction) -> Option<(GridIndex, Direction)> {
        let next_index = Self::next_index(direction, index)?;
        match self.grid.get(next_index) {
            Some(&Cell::Wall) => {
                let next_direction = direction.turn_right();
                Some((index, next_direction))
            }
            Some(&Cell::Empty) => Some((next_index, direction)),
            _ => None,
        }
    }

    fn next_index(direction: Direction, index: GridIndex) -> Option<GridIndex> {
        match direction {
            Direction::Up => index.up(),
            Direction::Right => index.right(),
            Direction::Down => index.down(),
            Direction::Left => index.left(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty,
    Guard(Direction),
    Wall,
}

impl Cell {
    pub fn as_guard(&self) -> Option<&Direction> {
        if let Self::Guard(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        }
    }
}

#[derive(Default)]
struct NextWall {
    up: Option<GridIndex>,
    right: Option<GridIndex>,
    down: Option<GridIndex>,
    left: Option<GridIndex>,
}

impl NextWall {
    fn new(
        up: Option<GridIndex>,
        right: Option<GridIndex>,
        down: Option<GridIndex>,
        left: Option<GridIndex>,
    ) -> Self {
        Self {
            up,
            right,
            down,
            left,
        }
    }
}
