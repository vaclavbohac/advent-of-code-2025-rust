use crate::chars::{DOT, ROLL};

const MAX_ACCESSIBLE_NEIGHBOURS: u32 = 4;

#[derive(Clone)]
pub struct Grid {
    cells: Vec<u8>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn new(height: usize, width: usize, cells: Vec<u8>) -> Grid {
        Grid {
            cells,
            height,
            width,
        }
    }

    pub fn for_each_paper<F: FnMut(usize, usize)>(&self, mut callback: F) {
        for row in 0..self.height {
            for column in 0..self.width {
                let cell = self.get(row, column);

                if cell == Some(ROLL) {
                    callback(row, column);
                }
            }
        }
    }

    pub fn from_str(s: &str) -> Self {
        let height = s.lines().count();
        let width = s.lines().next().map_or(0, str::len);
        let cells: Vec<u8> = s.lines().flat_map(str::bytes).collect();

        Grid::new(height, width, cells)
    }

    pub(crate) fn remove_paper_rolls(&mut self, paper_rolls: &[(usize, usize)]) {
        paper_rolls.iter().for_each(|&(row, column)| {
            self.set(row, column, DOT);
        });
    }

    pub fn get(&self, row: usize, column: usize) -> Option<u8> {
        if row >= self.height || column >= self.width {
            None
        } else {
            Some(self.cells[row * self.width + column])
        }
    }

    pub fn set(&mut self, row: usize, column: usize, value: u8) {
        if row >= self.height || column >= self.width {
            return;
        }

        self.cells[row * self.width + column] = value;
    }

    pub fn is_accessible(&self, row: usize, column: usize) -> bool {
        self.get(row, column) == Some(ROLL)
            && self.get_neighbors(row, column) < MAX_ACCESSIBLE_NEIGHBOURS
    }

    pub fn get_neighbors(&self, row: usize, column: usize) -> u32 {
        let mut paper_rolls = 0;

        (-1isize..=1)
            .flat_map(move |r| (-1isize..=1).map(move |c| (r, c)))
            .for_each(|(r, c)| {
                if r == 0 && c == 0 {
                    return;
                }

                let Some(n_row) = row.checked_add_signed(r) else {
                    return;
                };
                let Some(n_column) = column.checked_add_signed(c) else {
                    return;
                };

                if self.get(n_row, n_column) == Some(ROLL) {
                    paper_rolls += 1;
                }
            });

        paper_rolls
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    #[test]
    fn test_grid() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        assert_eq!(Some(b'.'), grid.get(0, 0));
        assert_eq!(Some(b'@'), grid.get(0, 2));
        assert_eq!(Some(b'.'), grid.get(1, 3));
    }

    #[test]
    fn test_is_accessible_for_roll_with_few_neighbors() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        // Top-edge roll with only 3 roll-neighbors (marked `x` in the puzzle).
        assert!(grid.is_accessible(0, 2));
    }

    #[test]
    fn test_is_not_accessible_for_roll_with_four_neighbors() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        // Roll with exactly 4 roll-neighbors stays put (`@`, not `x`).
        assert!(!grid.is_accessible(0, 7));
    }

    #[test]
    fn test_is_not_accessible_for_interior_roll() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        // Surrounded roll, well above the threshold.
        assert!(!grid.is_accessible(2, 3));
    }

    #[test]
    fn test_is_not_accessible_for_dot() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        // An empty cell is never accessible, regardless of its neighbors.
        assert_eq!(Some(b'.'), grid.get(0, 0));
        assert!(!grid.is_accessible(0, 0));
    }

    #[test]
    fn test_is_not_accessible_off_grid() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        assert!(!grid.is_accessible(grid.height, 0));
    }
}
