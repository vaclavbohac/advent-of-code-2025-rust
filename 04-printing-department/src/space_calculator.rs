const DOT: u8 = b'.';

pub fn count_accessible_paper_rolls(grid: Vec<Vec<char>>) -> u32 {
    let mut accessible_paper_rolls = 0;

    for (row_i, row) in grid.iter().enumerate() {
        for (column_i, column) in row.iter().enumerate() {
            if *column as u8 == DOT {
                continue;
            }

            let mut paper_rolls = 0;

            for i in -1i32..=1 {
                for j in -1i32..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let y: i32 = row_i as i32 + i;
                    let x: i32 = column_i as i32 + j;

                    if y < 0 || x < 0 {
                        continue
                    }

                    if y >= grid.len() as i32 || x >= row.len() as i32 {
                        continue
                    }

                    if grid[y as usize][x as usize] as u8 != DOT {
                        paper_rolls += 1;
                    }
                }
            }

            if paper_rolls < 4 {
                accessible_paper_rolls += 1;
            }
        }
    }

    accessible_paper_rolls
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    #[test]
    fn test_returns_number_of_accessible_paper_rolls() {
        let file_path = Path::new("resources/sample-input.txt");
        let f = File::open(file_path);

        let reader = BufReader::new(f.unwrap());
        let grid = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();

        assert_eq!(count_accessible_paper_rolls(grid), 13);
    }
}
