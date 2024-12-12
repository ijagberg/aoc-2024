use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use simple_grid::{Grid, GridIndex};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PlantMap {
    plants: Grid<Plant>,
    plant_to_region: Grid<usize>,
    regions: Vec<Region>,
}

impl PlantMap {
    pub fn new(grid: Grid<Plant>) -> Self {
        let (plant_to_region, regions) = Self::init_regions(&grid);
        Self {
            plants: grid,
            plant_to_region,
            regions,
        }
    }

    fn init_regions(plant_grid: &Grid<Plant>) -> (Grid<usize>, Vec<Region>) {
        let mut plant_to_region = Grid::new(
            plant_grid.width(),
            plant_grid.height(),
            vec![usize::MAX; plant_grid.area()],
        );
        let mut regions = Vec::new();

        for (idx, &plant) in plant_grid.cells_with_indices_iter() {
            if plant_to_region[idx] != usize::MAX {
                // already mapped the region for this plant
                continue;
            }

            regions.push(Region::new());
            let region_idx = regions.len() - 1;
            let region = regions.get_mut(region_idx).unwrap();
            let mut visited = HashSet::new();
            let mut to_visit = VecDeque::new();
            to_visit.push_back(idx);
            while let Some(current) = to_visit.pop_front() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);
                plant_to_region[current] = region_idx;
                region.cells_in_region.insert(current);

                for (neighbor_idx, perimeter_direction) in [
                    (current.up(), Direction::Up),
                    (current.right(), Direction::Right),
                    (current.down(), Direction::Down),
                    (current.left(), Direction::Left),
                ] {
                    if let Some(neighbor_idx) = neighbor_idx {
                        if plant_grid.contains_index(neighbor_idx) {
                            let neighbor_plant = plant_grid[neighbor_idx];
                            if neighbor_plant == plant && !visited.contains(&neighbor_idx) {
                                to_visit.push_back(neighbor_idx);
                            } else if neighbor_plant != plant {
                                region.perimeter.insert((current, perimeter_direction));
                            }
                        } else {
                            region.perimeter.insert((current, perimeter_direction));
                        }
                    } else {
                        region.perimeter.insert((current, perimeter_direction));
                    }
                }
            }
        }

        (plant_to_region, regions)
    }

    pub(crate) fn plants(&self) -> &Grid<Plant> {
        &self.plants
    }

    pub(crate) fn plant_to_region(&self) -> &Grid<usize> {
        &self.plant_to_region
    }

    pub fn regions(&self) -> &Vec<Region> {
        &self.regions
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Plant(char);

impl Plant {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

impl Display for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Region {
    cells_in_region: HashSet<GridIndex>,
    perimeter: HashSet<(GridIndex, Direction)>,
}

impl Region {
    pub fn new() -> Self {
        Self {
            cells_in_region: HashSet::new(),
            perimeter: HashSet::new(),
        }
    }

    pub fn area(&self) -> usize {
        self.cells_in_region.len()
    }

    pub fn perimeter(&self) -> &HashSet<(GridIndex, Direction)> {
        &self.perimeter
    }

    pub fn sides(&self) -> usize {
        let mut ups = HashMap::new();
        let mut rights = HashMap::new();
        let mut downs = HashMap::new();
        let mut lefts = HashMap::new();
        for &(idx, direction) in self.perimeter() {
            match direction {
                Direction::Up => ups.entry(idx.row()).or_insert(Vec::new()).push(idx),
                Direction::Right => rights.entry(idx.column()).or_insert(Vec::new()).push(idx),
                Direction::Down => downs.entry(idx.row()).or_insert(Vec::new()).push(idx),
                Direction::Left => lefts.entry(idx.column()).or_insert(Vec::new()).push(idx),
            }
        }

        let mut sides = 0;

        for map in [ups, downs] {
            for (row, mut indices) in map {
                sides += 1;
                indices.sort_by_key(|i| i.column());
                for w in indices.windows(2) {
                    // find non-contiguous fences
                    if w[0].column() != w[1].column() - 1 {
                        sides += 1;
                    }
                }
            }
        }

        for map in [rights, lefts] {
            for (column, mut indices) in map {
                sides += 1;
                indices.sort_by_key(|i| i.row());
                for w in indices.windows(2) {
                    // find non-contiguous fences
                    if w[0].row() != w[1].row() - 1 {
                        sides += 1;
                    }
                }
            }
        }

        sides
    }

    fn next_index(idx: GridIndex, direction: Direction) -> GridIndex {
        match direction {
            Direction::Up => idx.up(),
            Direction::Right => idx.right(),
            Direction::Down => idx.down(),
            Direction::Left => idx.left(),
        }
        .unwrap()
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

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Right => Self::Up,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
        }
    }
}
