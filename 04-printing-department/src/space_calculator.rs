use crate::grid::Grid;

pub fn count_accessible_paper_rolls(grid: &Grid) -> u32 {
    let mut accessible_paper_rolls = 0;

    grid.for_each_paper(|row_i, column_i| {
        if grid.is_accessible(row_i, column_i) {
            accessible_paper_rolls += 1;
        }
    });

    accessible_paper_rolls
}

pub fn count_removable_paper_rolls(grid: &Grid) -> u32 {
    let mut removable_paper_rolls_total: u32 = 0;
    let mut cloned = grid.clone();

    loop {
        let mut removable_paper_rolls: Vec<(usize, usize)> = Vec::new();

        cloned.for_each_paper(|row_i, column_i| {
            if !cloned.is_accessible(row_i, column_i) {
                return;
            }

            removable_paper_rolls.push((row_i, column_i))
        });

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
