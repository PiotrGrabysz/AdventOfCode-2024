use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();
        let mut safe_lines_count = 0;
        for line in lines {
            let levels = convert_line_to_numbers(&line);
            if are_all_levels_safe(levels) {
                safe_lines_count += 1;
            }
        }
        Ok(safe_lines_count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let line_count = count_lines(BufReader::new(File::open(INPUT_FILE)?));
    println!("Number of lines in the file: {}", line_count);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Brute force solution
        let mut safe_lines_count = 0;
        let lines = reader.lines().flatten();
        for line in lines {
            let levels = convert_line_to_numbers(&line);
            for index_to_drop in 0..levels.len() {
                let mut levels_without_one_index = levels.clone();
                levels_without_one_index.remove(index_to_drop);

                if are_all_levels_safe(levels_without_one_index) {
                    safe_lines_count += 1;
                    break;
                }
            }
        }
        Ok(safe_lines_count)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn convert_line_to_numbers(line: &str) -> Vec<i32> {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|x| {
            x.parse::<i32>()
                .expect("The input data contains a valid list of numbers")
        })
        .collect();
    numbers
}

fn are_all_levels_safe(levels: Vec<i32>) -> bool {
    if adjacent_levels_have_incorrect_difference(&levels) {
        return false;
    }
    if !levels_are_monotonic(&levels) {
        return false;
    }
    true
}

fn count_lines<R: BufRead>(reader: R) -> usize {
    let lines = reader.lines().flatten();
    let count = lines.count();
    count
}

fn adjacent_levels_have_incorrect_difference(levels: &Vec<i32>) -> bool {
    let min_difference = 1;
    let max_difference = 3;

    for window in levels.windows(2) {
        if let [x, y] = window {
            let difference = (x - y).abs();
            if difference < min_difference || difference > max_difference {
                return true;
            }
        };
    }
    false
}

fn levels_are_monotonic(levels: &Vec<i32>) -> bool {
    if levels.is_sorted() {
        return true;
    }
    if levels.iter().rev().is_sorted() {
        return true;
    }
    false
}
