use simple_grid::{Grid, GridIndex};

pub struct BoxMap {
    grid: Grid<Cell>,
    robot: GridIndex,
}

impl BoxMap {
    pub fn new(grid: Grid<Cell>) -> Self {
        let mut robots: Vec<_> = grid
            .cells_with_indices_iter()
            .filter(|(i, &c)| c == Cell::Robot)
            .map(|(i, _)| i)
            .collect();

        assert_eq!(robots.len(), 1);

        Self {
            grid,
            robot: robots.remove(0),
        }
    }

    pub fn step(&mut self, direction: Direction) {
        if let Some(new_robot_index) = self.move_object(self.robot, direction) {
            self.robot = new_robot_index;
        }
    }

    pub fn box_indices(&self) -> impl Iterator<Item = GridIndex> + use<'_> {
        self.grid
            .cells_with_indices_iter()
            .filter_map(|(i, &c)| if c == Cell::Box { Some(i) } else { None })
    }

    fn move_object(&mut self, from_cell: GridIndex, direction: Direction) -> Option<GridIndex> {
        let to_cell = match direction {
            Direction::Up => from_cell.up(),
            Direction::Right => from_cell.right(),
            Direction::Down => from_cell.down(),
            Direction::Left => from_cell.left(),
        }?;
        if !self.grid.contains_index(to_cell) {
            return None;
        }

        match self.grid[to_cell] {
            Cell::Robot => unreachable!("this shouldn't happen"),
            Cell::Box => {
                // try to move the box further
                if let Some(_) = self.move_object(to_cell, direction) {
                    self.grid.swap_cells(to_cell, from_cell);
                    return Some(to_cell);
                } else {
                    return None;
                }
            }
            Cell::Empty => {
                self.grid.swap_cells(to_cell, from_cell);
                return Some(to_cell);
            }
            Cell::Wall => return None,
        }
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Robot,
    Box,
    Empty,
    Wall,
}

pub struct WideBoxMap {
    grid: Grid<WideCell>,
    robot: GridIndex,
}

impl WideBoxMap {
    pub fn new(grid: Grid<WideCell>) -> Self {
        let mut robots: Vec<_> = grid
            .cells_with_indices_iter()
            .filter(|(i, &c)| c == WideCell::Robot)
            .map(|(i, _)| i)
            .collect();

        assert_eq!(robots.len(), 1);

        Self {
            grid,
            robot: robots.remove(0),
        }
    }

    pub fn step(&mut self, direction: Direction) {
        if let Some(new_robot_index) = self.move_object(self.robot, direction) {
            self.robot = new_robot_index;
        }
    }

    pub fn box_indices(&self) -> impl Iterator<Item = GridIndex> + use<'_> {
        self.grid.cells_with_indices_iter().filter_map(|(i, &c)| {
            if c == WideCell::WideBoxLeft {
                Some(i)
            } else {
                None
            }
        })
    }

    fn move_object(&mut self, from_cell: GridIndex, direction: Direction) -> Option<GridIndex> {
        let to_cell = match direction {
            Direction::Up => from_cell.up(),
            Direction::Right => from_cell.right(),
            Direction::Down => from_cell.down(),
            Direction::Left => from_cell.left(),
        }?;
        if !self.grid.contains_index(to_cell) {
            return None;
        }

        match self.grid[to_cell] {
            WideCell::Robot => unreachable!("this shouldn't happen"),
            WideCell::WideBoxLeft => {
                // try to move the box further
                if let Some(_) = self.move_object(to_cell, direction) {
                    self.grid.swap_cells(to_cell, from_cell);
                    return Some(to_cell);
                } else {
                    return None;
                }
            }
            WideCell::WideBoxRight => {
                // try to move the box further
                if let Some(_) = self.move_object(to_cell, direction) {
                    self.grid.swap_cells(to_cell, from_cell);
                    return Some(to_cell);
                } else {
                    return None;
                }
            }
            WideCell::Empty => {
                self.grid.swap_cells(to_cell, from_cell);
                return Some(to_cell);
            }
            WideCell::Wall => return None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WideCell {
    Robot,
    WideBoxLeft,
    WideBoxRight,
    Empty,
    Wall,
}
