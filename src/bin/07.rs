use adv_code_2024::number_utils::concatenate_numbers;
use adv_code_2024::start_day;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let mut answer = 0;
        let possible_operations = vec![Operation::Add, Operation::Multiply];
        for line in reader.lines().flatten() {
            let (test_value, numbers) = read_equation(&line)?;
            if is_equation_true(test_value, &numbers, &possible_operations) {
                answer += test_value;
            }
        }
        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut answer = 0;
        let possible_operations = vec![Operation::Add, Operation::Multiply, Operation::Concatenate];
        for line in reader.lines().flatten() {
            let (test_value, numbers) = read_equation(&line)?;
            if is_equation_true(test_value, &numbers, &possible_operations) {
                answer += test_value;
            }
        }
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read_equation(line: &str) -> Result<(u64, Vec<u64>)> {
    let mut split_line = line.trim().split(':');
    let test_value = split_line.next().unwrap().parse::<u64>()?;
    let numbers = _read_numbers(split_line.next().unwrap())?;
    Ok((test_value, numbers))
}

fn _read_numbers(numbers_str: &str) -> Result<Vec<u64>> {
    let numbers: Vec<u64> = numbers_str
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    Ok(numbers)
}

fn is_equation_true(
    test_value: u64,
    numbers: &Vec<u64>,
    possible_operations: &Vec<Operation>,
) -> bool {
    let first_number = numbers[0];
    let numbers_rest = &numbers[1..];

    let number_of_operations = numbers_rest.len();

    let combos: Vec<_> = (0..number_of_operations)
        .map(|_| possible_operations.iter().cloned()) // Repeat the iterator r times
        .multi_cartesian_product()
        .collect();

    for combo in combos {
        let mut result = first_number;
        for (num, operation) in numbers_rest.iter().zip(combo) {
            match operation {
                Operation::Add => result += num,
                Operation::Multiply => result *= num,
                Operation::Concatenate => result = concatenate_numbers(result, *num),
            }
        }
        if result == test_value {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}
