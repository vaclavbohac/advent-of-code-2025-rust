use crate::chars::DOT;
use crate::grid::Grid;

const MAX_ACCESSIBLE_NEIGHBOURS: u32 = 4;

pub fn count_accessible_paper_rolls(grid: &Grid) -> u32 {
    let mut accessible_paper_rolls = 0;

    for row_i in 0..grid.height() {
        for cell_i in 0..grid.width() {
            let cell = grid.get(cell_i, row_i);

            if cell == Some(DOT) {
                continue;
            }

            let paper_rolls = grid.get_neighbors(row_i, cell_i);

            if paper_rolls < MAX_ACCESSIBLE_NEIGHBOURS {
                accessible_paper_rolls += 1;
            }
        }
    }

    accessible_paper_rolls
}

pub fn count_removable_paper_rolls(grid: &Grid) -> u32 {
    let mut removable_paper_rolls_total: u32 = 0;
    let mut cloned = grid.clone();

    loop {
        let mut removable_paper_rolls: Vec<(usize, usize)> = Vec::new();

        for row_i in 0..cloned.height() {
            for cell_i in 0..cloned.width() {
                let cell = cloned.get(cell_i, row_i);

                if cell == Some(DOT) {
                    continue;
                }

                let paper_rolls = cloned.get_neighbors(row_i, cell_i);

                if paper_rolls >= MAX_ACCESSIBLE_NEIGHBOURS {
                    continue;
                }

                removable_paper_rolls.push((row_i, cell_i))
            }
        }

        if removable_paper_rolls.is_empty() {
            break;
        }

        removable_paper_rolls_total += removable_paper_rolls.len() as u32;

        cloned.remove_paper_rolls(&removable_paper_rolls);
    }

    removable_paper_rolls_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_number_of_accessible_paper_rolls() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        assert_eq!(count_accessible_paper_rolls(&grid), 13);
    }

    #[test]
    fn test_returns_number_of_removable_paper_rolls() {
        let input = include_str!("../resources/sample-input.txt");

        let grid = Grid::from_str(input);

        assert_eq!(count_removable_paper_rolls(&grid), 43);
    }
}
