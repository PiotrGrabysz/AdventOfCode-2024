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
// const TEST2: &str = "\
// mul(2,4)don't()mul(1,3)don't()mul(11,33)do()mul(22,44)do()
// ";
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: &mut R) -> Result<i32> {
        // Capture patterns like "mul(x,y)", where "x" and "y" are numbers up to three digits
        let multiplication_formula_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

        let file_content = read_file_to_string(reader);

        let multiplication_sum = multiplication_formula_regex
            .captures_iter(file_content.as_str())
            .filter_map(|captures| {
                let x = captures.get(1)?.as_str().parse::<i32>().ok()?;
                let y = captures.get(2)?.as_str().parse::<i32>().ok()?;
                Some(x * y)
            })
            .sum();

        Ok(multiplication_sum)
    }

    assert_eq!(161, part1(&mut BufReader::new(TEST.as_bytes()))?);

    let mut input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(&mut input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: &mut R) -> Result<i32> {
        let disabled_operations_regex = Regex::new(r"don't\(\)")?;
        let enabled_operations_regex = Regex::new(r"do\(\)")?;
        let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)$")?;
        let extract_numbers_regex = Regex::new(r"\d{1,3}")?;

        let mut file_content: String = String::new();
        _ = reader.read_to_string(&mut file_content);

        let mut multiplication_sum = 0;

        let mut current_string: String = String::new();
        let mut disabled_operations = false;
        for char in file_content.chars() {
            // dbg!(char);
            current_string.push(char);
            // dbg!(&current_string);
            if disabled_operations_regex.is_match(&current_string) {
                disabled_operations = true;
                current_string.clear();
                continue;
            }
            if enabled_operations_regex.is_match(&current_string) {
                disabled_operations = false;
                current_string.clear();
                continue;
            }
            if !disabled_operations {
                if let Some(res) = mul_regex.find_iter(&current_string).last() {
                    let numbers: Vec<i32> = extract_numbers_regex
                        .find_iter(res.as_str())
                        .filter_map(|m| m.as_str().parse::<i32>().ok())
                        .collect();
                    // dbg!("Adding {} * {}",  numbers[0], numbers[1]);
                    multiplication_sum += numbers[0] * numbers[1];
                    current_string.clear();
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
