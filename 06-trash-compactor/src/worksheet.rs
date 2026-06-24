use std::mem::take;

#[derive(Debug, PartialEq)]
pub struct Worksheet {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl Worksheet {
    pub fn new(numbers: Vec<Vec<u64>>, operations: Vec<Operation>) -> Self {
        assert_eq!(numbers.len(), operations.len());

        Worksheet {
            numbers,
            operations,
        }
    }

    pub fn calculate(&self) -> u64 {
        self.operations
            .iter()
            .enumerate()
            .map(|(column, operation)| match operation {
                Operation::Plus => self.addition(column),
                Operation::Multiply => self.multiplication(column),
            })
            .sum()
    }

    fn multiplication(&self, column: usize) -> u64 {
        self.numbers[column].iter().product()
    }

    fn addition(&self, column: usize) -> u64 {
        self.numbers[column].iter().sum()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Plus,
    Multiply,
}

fn parse_operation(token: &str) -> Operation {
    match token {
        "+" => Operation::Plus,
        "*" => Operation::Multiply,
        _ => panic!("Unknown operation: {}", token),
    }
}

pub fn parse_worksheet(input: &str) -> Worksheet {
    let mut worksheet_numbers: Vec<Vec<u64>> = Vec::new();
    let mut worksheet_operations: Vec<Operation> = Vec::new();

    let mut rows: Vec<&str> = input.lines().collect();
    rows.pop();

    for (operation, (from, to)) in seek_boundaries(input) {
        let mut numbers: Vec<u64> = Vec::new();

        for row in rows.iter() {
            numbers.push(
                row[from..=to.min(row.len() - 1)]
                    .trim()
                    .parse::<u64>()
                    .unwrap(),
            );
        }

        worksheet_numbers.push(numbers);
        worksheet_operations.push(operation);
    }

    Worksheet::new(worksheet_numbers, worksheet_operations)
}

fn seek_boundaries(input: &str) -> Vec<(Operation, (usize, usize))> {
    let Some(last_line) = input.lines().last() else {
        return vec![];
    };

    let max_length = input.lines().max_by_key(|l| l.len()).unwrap().len();

    let mut vec: Vec<(Operation, (usize, usize))> = Vec::new();

    let mut current_operation: Option<Operation> = None;
    let mut current_index_start: usize = 0;

    for (i, cell) in last_line.bytes().enumerate() {
        if cell == b' ' {
            continue;
        }

        let op = parse_operation(&last_line[i..=i]);
        if current_operation.is_none() {
            current_operation = Some(op);
            current_index_start = i;
        } else {
            // i - 2 is to skip the whitespace between the boxes of numbers
            vec.push((current_operation.unwrap(), (current_index_start, i - 2)));
            current_operation = Some(op);
            current_index_start = i;
        }
    }

    vec.push((
        current_operation.unwrap(),
        (current_index_start, max_length - 1),
    ));

    vec
}

pub fn parse_worksheet_vertically(input: &str) -> Worksheet {
    let mut worksheet_numbers: Vec<Vec<u64>> = Vec::new();
    let mut worksheet_operations: Vec<Operation> = Vec::new();

    let mut rows: Vec<&str> = input.lines().collect();
    rows.pop();

    for (operation, (from, to)) in seek_boundaries(input) {
        worksheet_operations.push(operation);

        let mut numbers: Vec<u64> = Vec::new();
        let mut n: Vec<u8> = Vec::new();

        for column_i in (from..=to).rev() {
            for row in rows.iter() {
                let Some(&byte) = row.as_bytes().get(column_i) else {
                    continue;
                };

                if byte != b' ' {
                    n.push(byte);
                }
            }

            let s = String::from_utf8(take(&mut n)).unwrap();
            numbers.push(s.parse::<u64>().unwrap());
        }

        worksheet_numbers.push(numbers);
    }

    Worksheet::new(worksheet_numbers, worksheet_operations)
}

#[cfg(test)]
mod tests {
    use crate::worksheet::{
        Operation, Worksheet, parse_worksheet, parse_worksheet_vertically, seek_boundaries,
    };

    #[test]
    fn test_parse_worksheet() {
        let input = include_str!("../resources/sample-input.txt");

        let expected_numbers = vec![
            vec![123, 45, 6],
            vec![328, 64, 98],
            vec![51, 387, 215],
            vec![64, 23, 314],
        ];

        let expected_operations = vec![
            Operation::Multiply,
            Operation::Plus,
            Operation::Multiply,
            Operation::Plus,
        ];

        let expected_worksheet = Worksheet::new(expected_numbers, expected_operations);

        let worksheet = parse_worksheet(input);

        assert_eq!(expected_worksheet, worksheet);
    }

    #[test]
    fn test_worksheet_calculations() {
        let input = include_str!("../resources/sample-input.txt");

        let worksheet = parse_worksheet(input);

        assert_eq!(4277556, worksheet.calculate());
    }

    #[test]
    fn test_parse_worksheet_vertically() {
        let input = include_str!("../resources/sample-input.txt");

        let expected_numbers = vec![
            vec![356, 24, 1],
            vec![8, 248, 369],
            vec![175, 581, 32],
            vec![4, 431, 623],
        ];

        let expected_operations = vec![
            Operation::Multiply,
            Operation::Plus,
            Operation::Multiply,
            Operation::Plus,
        ];

        let expected_worksheet = Worksheet::new(expected_numbers, expected_operations);

        let worksheet = parse_worksheet_vertically(input);

        assert_eq!(expected_worksheet, worksheet);
    }

    #[test]
    fn test_seek_boundaries() {
        let input = include_str!("../resources/sample-input.txt");

        let expected = vec![
            (Operation::Multiply, (0, 2)),
            (Operation::Plus, (4, 6)),
            (Operation::Multiply, (8, 10)),
            (Operation::Plus, (12, 14)),
        ];

        assert_eq!(seek_boundaries(input), expected);
    }
}
