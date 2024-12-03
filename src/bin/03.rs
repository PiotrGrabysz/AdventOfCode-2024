use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: &mut R) -> Result<i32> {
        // Capture patterns like "mul(x,y)", where "x" and "y" are numbers up to three digits

        let file_content = read_file_to_string(reader);

        sum_mul_operations(file_content.as_str())
    }

    assert_eq!(161, part1(&mut BufReader::new(TEST.as_bytes()))?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(&mut input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: &mut R) -> Result<i32> {
        let multiplication_formula_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let disable_instruction_regex = Regex::new(r"don't\(\)")?;
        let enable_instruction_regex = Regex::new(r"do\(\)")?;

        let file_content = read_file_to_string(reader);

        let mut multiplication_sum = 0;
        let mut disabled_instruction_mode = false;
        let mut current_string: String = String::new();
        for char in file_content.chars() {
            current_string.push(char);

            if disable_instruction_regex.is_match(&current_string) {
                disabled_instruction_mode = true;
                current_string.clear();
                continue;
            }

            if enable_instruction_regex.is_match(&current_string) {
                disabled_instruction_mode = false;
                current_string.clear();
                continue;
            }
            if !disabled_instruction_mode {
                match get_last_multiplication(current_string.as_str(), &multiplication_formula_regex) {
                    Some(m) => {
                        multiplication_sum += m;
                        current_string.clear();
                    }
                    None => continue,
                }
            }
        }
        Ok(multiplication_sum)
    }

    assert_eq!(48, part2(&mut BufReader::new(TEST2.as_bytes()))?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(&mut input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read_file_to_string<R: BufRead>(reader: &mut R) -> String {
    let mut file_content: String = String::new();
    _ = reader.read_to_string(&mut file_content);
    file_content
}

fn sum_mul_operations(instructions: &str) -> Result<i32> {
    let multiplication_formula_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let multiplication_sum: i32 = multiplication_formula_regex
        .captures_iter(instructions)
        .filter_map(|captures| {
            let x = captures.get(1)?.as_str().parse::<i32>().ok()?;
            let y = captures.get(2)?.as_str().parse::<i32>().ok()?;
            Some(x * y)
        })
        .sum();
    Ok(multiplication_sum)
}

fn get_last_multiplication(instructions: &str, multiplication_formula_regex: &Regex) -> Option<i32> {
    if let Some(captures) = multiplication_formula_regex
        .captures_iter(instructions)
        .last()
    {
        let num1 = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .ok()
            .unwrap();
        let num2 = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .ok()
            .unwrap();
        return Some(num1 * num2)
    }
    None
}
