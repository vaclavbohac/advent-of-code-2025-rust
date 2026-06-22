use crate::chars::ROLL;

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

    pub fn from_str(s: &str) -> Self {
        let height = s.lines().count();
        let width = s.lines().next().map_or(0, str::len);
        let cells: Vec<u8> = s.lines().flat_map(str::bytes).collect();

        Grid::new(height, width, cells)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.cells[y * self.width + x])
        }
    }

    pub fn get_neighbors(&self, row: usize, column: usize) -> u32 {
        let mut paper_rolls = 0;

        (-1isize..=1).for_each(|i| {
            (-1isize..=1).for_each(|j| {
                if i == 0 && j == 0 {
                    return;
                }

                let Some(x) = column.checked_add_signed(i) else {
                    return;
                };
                let Some(y) = row.checked_add_signed(j) else {
                    return;
                };

                if self.get(x, y) == Some(ROLL) {
                    paper_rolls += 1;
                }
            })
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
        assert_eq!(Some(b'@'), grid.get(2, 0));
        assert_eq!(Some(b'.'), grid.get(3, 1));
    }
}
