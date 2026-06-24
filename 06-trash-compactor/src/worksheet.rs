#[derive(Debug, PartialEq)]
pub struct Worksheet {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl Worksheet {
    pub fn new(numbers: Vec<Vec<u64>>, operations: Vec<Operation>) -> Self {
        Worksheet {
            numbers,
            operations,
        }
    }

    pub fn calculate(&self) -> u64 {
        let mut sum: u64 = 0;

        for (column, operation) in self.operations.iter().enumerate() {
            let result = match operation {
                Operation::Plus => self.addition(column),
                Operation::Multiply => self.multiplication(column),
                Operation::Minus => 0,
                Operation::Divide => 0,
            };

            sum += result;
        }

        sum
    }

    fn multiplication(&self, column: usize) -> u64 {
        let mut result: u64 = 1;
        for row in &self.numbers {
            result *= row[column];
        }
        result
    }

    fn addition(&self, column: usize) -> u64 {
        let mut result: u64 = 0;
        for row in &self.numbers {
            result += row[column];
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn parse_operation(token: &str) -> Operation {
    if token == "+" {
        Operation::Plus
    } else if token == "-" {
        Operation::Minus
    } else if token == "/" {
        Operation::Divide
    } else if token == "*" {
        Operation::Multiply
    } else {
        panic!("Unknown operation: {}", token);
    }
}

pub fn parse_worksheet(input: &str) -> Worksheet {
    let mut worksheet_numbers: Vec<Vec<u64>> = Vec::new();
    let mut worksheet_operations: Vec<Operation> = Vec::new();

    for line in input.lines() {
        let tokens = line.trim().split(' ').collect::<Vec<&str>>();

        let mut numbers: Vec<u64> = Vec::new();
        for token in tokens {
            if token.is_empty() {
                continue;
            }

            let result = token.parse::<u64>();
            match result {
                Ok(n) => numbers.push(n),
                Err(_) => {
                    let op = parse_operation(token);
                    worksheet_operations.push(op);
                }
            }
        }

        if !numbers.is_empty() {
            worksheet_numbers.push(numbers)
        }
    }

    Worksheet::new(worksheet_numbers, worksheet_operations)
}

#[cfg(test)]
mod tests {
    use crate::worksheet::{Operation, Worksheet, parse_worksheet};

    #[test]
    fn test_parse_worksheet() {
        let input = include_str!("../resources/sample-input.txt");

        let expected_numbers = vec![
            vec![123, 328, 51, 64],
            vec![45, 64, 387, 23],
            vec![6, 98, 215, 314],
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
}
