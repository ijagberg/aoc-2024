use simple_grid::{Grid, GridIndex};
use std::collections::{HashMap, HashSet};

pub struct AntennaMap {
    antennas: Grid<Option<Antenna>>,
}

impl AntennaMap {
    pub fn new(antennas: Grid<Option<Antenna>>) -> Self {
        Self { antennas }
    }

    pub fn get_antinodes(&self) -> Grid<Vec<char>> {
        let mut antinodes: Grid<Vec<char>> =
            Grid::new_default(self.antennas.width(), self.antennas.height());

        let antennas = self.antennas_by_frequency();
        for (frequency, antennas_with_frequency) in antennas {
            for &a in &antennas_with_frequency {
                for &b in &antennas_with_frequency {
                    if a == b {
                        continue;
                    }

                    if let Some(antinode) = self.antinode_of(a, b) {
                        antinodes[antinode].push(frequency);
                    }
                    if let Some(antinode) = self.antinode_of(b, a) {
                        antinodes[antinode].push(frequency);
                    }
                }
            }
        }

        antinodes
    }

    pub fn get_resonant_antinodes(&self) -> Grid<Vec<char>> {
        let mut antinodes: Grid<Vec<char>> =
            Grid::new_default(self.antennas.width(), self.antennas.height());

        let antennas = self.antennas_by_frequency();
        for (frequency, antennas_with_frequency) in antennas {
            for &a in &antennas_with_frequency {
                for &b in &antennas_with_frequency {
                    if a == b {
                        continue;
                    }

                    for resonant_antinode in self.resonant_antinodes_of(a, b) {
                        antinodes[resonant_antinode].push(frequency);
                    }
                    for resonant_antinode in self.resonant_antinodes_of(b, a) {
                        antinodes[resonant_antinode].push(frequency);
                    }
                }
            }
        }

        antinodes
    }

    fn antennas_by_frequency(&self) -> HashMap<char, HashSet<GridIndex>> {
        let mut map: HashMap<char, HashSet<GridIndex>> = HashMap::new();

        for (idx, cell) in self.antennas.cells_with_indices_iter() {
            if let Some(f) = cell {
                map.entry(f.frequency).or_default().insert(idx);
            }
        }

        map
    }

    fn antinode_of(&self, from: GridIndex, to: GridIndex) -> Option<GridIndex> {
        let x_dist = from.column() as isize - to.column() as isize;
        let y_dist = from.row() as isize - to.row() as isize;

        let antinode_column = from.column() as isize + x_dist;
        let antinode_row = from.row() as isize + y_dist;

        self.within_bounds(antinode_column, antinode_row)
    }

    fn resonant_antinodes_of(&self, from: GridIndex, to: GridIndex) -> Vec<GridIndex> {
        let x_dist = from.column() as isize - to.column() as isize;
        let y_dist = from.row() as isize - to.row() as isize;

        let mut antinodes = Vec::new();
        for step in 0.. {
            let antinode_column = from.column() as isize + (step * x_dist);
            let antinode_row = from.row() as isize + (step * y_dist);

            if let Some(antinode_idx) = self.within_bounds(antinode_column, antinode_row) {
                antinodes.push(antinode_idx);
            } else {
                return antinodes;
            }
        }

        unreachable!()
    }

    fn within_bounds(&self, column: isize, row: isize) -> Option<GridIndex> {
        if column < 0
            || row < 0
            || column >= self.antennas.width() as isize
            || row >= self.antennas.height() as isize
        {
            None
        } else {
            Some(GridIndex::new(column as usize, row as usize))
        }
    }
}

pub struct Antenna {
    frequency: char,
}

impl Antenna {
    pub fn new(frequency: char) -> Self {
        Self { frequency }
    }
}
