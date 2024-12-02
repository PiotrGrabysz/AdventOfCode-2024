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
        .map(|x| x.parse::<i32>().expect("The input data contains a valid list of numbers"))
        .collect();
    numbers
}

fn are_all_levels_safe(levels: Vec<i32>) -> bool {

    let mut maybe_increasing = true;
    let mut maybe_decreasing = true;

    let mut prev_number = &levels[0];
    for next_number in &levels[1..] {
        let difference = (next_number - prev_number).abs();
        if difference < 1 || difference > 3 {
            return false;
        }

        if prev_number < next_number {
            maybe_decreasing = false;
        }
        if prev_number > next_number {
            maybe_increasing = false;
        }

        prev_number = next_number;
    }
    maybe_increasing || maybe_decreasing
}
