use simple_grid::{Grid, GridIndex};

pub struct WordSearch {
    grid: Grid<char>,
}

impl WordSearch {
    pub fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    pub fn find_all_crosses(&self, word: &[char]) -> Vec<GridIndex> {
        if word.len() % 2 != 1 {
            return Vec::new();
        }
        let mut matches = Vec::new();
        for row in self.grid.rows() {
            for column in self.grid.columns() {
                let idx = GridIndex::new(column, row);
                if self.find_cross_at(word, idx) {
                    matches.push(idx);
                }
            }
        }

        matches
    }

    fn find_cross_at(&self, word: &[char], index: GridIndex) -> bool {
        let half_word = word.len() / 2;
        if index.column() + half_word >= self.grid.width()
            || index.column() < half_word
            || index.row() + half_word >= self.grid.height()
            || index.row() < half_word
        {
            // not within bounds
            return false;
        }

        // check top left to bottom right diagonal
        let indices = (index.row() - half_word..=index.row() + half_word)
            .zip(index.column() - half_word..=index.column() + half_word);
        if !(indices
            .clone()
            .zip(word)
            .all(|((column, row), c)| self.grid[GridIndex::new(column, row)] == *c)
            || indices
                .zip(word.iter().rev())
                .all(|((column, row), c)| self.grid[GridIndex::new(column, row)] == *c))
        {
            return false;
        }

        // check bottom left to top right diagonal
        let indices = (index.row() - half_word..=index.row() + half_word)
            .zip((index.column() - half_word..=index.column() + half_word).rev());
        if !(indices
            .clone()
            .zip(word)
            .all(|((column, row), c)| self.grid[GridIndex::new(column, row)] == *c)
            || indices
                .zip(word.iter().rev())
                .all(|((column, row), c)| self.grid[GridIndex::new(column, row)] == *c))
        {
            return false;
        }

        true
    }

    pub fn find_all_words(&self, word: &[char]) -> Vec<Vec<GridIndex>> {
        let mut matches = Vec::new();
        for row in self.grid.rows() {
            for column in self.grid.columns() {
                let idx = GridIndex::new(column, row);
                let mut index_matches = self.check_single_start(idx, word);
                matches.append(&mut index_matches);
            }
        }

        matches
    }

    fn check_single_start(&self, index: GridIndex, word: &[char]) -> Vec<Vec<GridIndex>> {
        let mut matches = Vec::new();
        // left to right
        if index.column() + word.len() <= self.grid.width() {
            let mut indices = Vec::with_capacity(word.len());
            for c in 0..word.len() {
                indices.push(GridIndex::new(index.column() + c, index.row()));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        // right to left
        if index.column() >= word.len() - 1 {
            let mut indices = Vec::with_capacity(word.len());
            for c in 0..word.len() {
                indices.push(GridIndex::new(index.column() - c, index.row()));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        // top to bottom
        if index.row() + word.len() <= self.grid.height() {
            let mut indices = Vec::with_capacity(word.len());
            for r in 0..word.len() {
                indices.push(GridIndex::new(index.column(), index.row() + r));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        // bottom to top
        if index.row() >= word.len() - 1 {
            let mut indices = Vec::with_capacity(word.len());
            for r in 0..word.len() {
                indices.push(GridIndex::new(index.column(), index.row() - r));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        // top left to bottom right
        if index.column() + word.len() <= self.grid.width()
            && index.row() + word.len() <= self.grid.height()
        {
            let mut indices = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                indices.push(GridIndex::new(index.column() + i, index.row() + i));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        // bottom right to top left
        if index.column() >= word.len() - 1 && index.row() >= word.len() - 1 {
            let mut indices = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                indices.push(GridIndex::new(index.column() - i, index.row() - i));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        // top right to bottom left
        if index.column() >= word.len() - 1 && index.row() + word.len() <= self.grid.height() {
            let mut indices = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                indices.push(GridIndex::new(index.column() - i, index.row() + i));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        // bottom left to top right
        if index.column() + word.len() <= self.grid.width() && index.row() >= word.len() - 1 {
            let mut indices = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                indices.push(GridIndex::new(index.column() + i, index.row() - i));
            }
            if self.check_indices(&indices, word) {
                matches.push(indices);
            }
        }

        matches
    }

    fn check_indices(&self, indices: &[GridIndex], word: &[char]) -> bool {
        if indices.len() != word.len() {
            false
        } else {
            indices
                .iter()
                .zip(word.iter())
                .all(|(a, b)| self.grid.get(*a).unwrap() == b)
        }
    }
}
