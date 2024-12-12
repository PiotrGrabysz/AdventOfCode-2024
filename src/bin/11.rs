use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let number_of_blinks = 25;
        let mut stones: Vec<u64> = read_stones(reader);

        blink_n_times_naive(&mut stones, number_of_blinks);
        Ok(stones.len().try_into()?)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let number_of_blinks = 75;

        let stones: Vec<u64> = read_stones(reader);

        let answer = recursive_solution(&stones, number_of_blinks);

        Ok(answer)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn read_stones<R: BufRead>(reader: R) -> Vec<u64> {
    reader
        .lines()
        .flatten()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn blink_n_times_naive(stones: &mut Vec<u64>, number_of_blinks: u8) -> () {
    for idx in 0..number_of_blinks {
        if idx % 25 == 0 {
            println!("Blink {} / {}", idx, number_of_blinks);
        }
        let mut right_items = Vec::new();
        for stone in &mut *stones {
            if *stone == 0 {
                *stone = 1;
                continue;
            }
            let num_digits = (*stone as f64).log10().floor() as u32 + 1; // Calculate number of digits
            if num_digits % 2 == 0 {
                let divisor = 10_u64.pow(num_digits / 2); // Find the split divisor
                let left = *stone / divisor; // Get the left part
                let right = *stone % divisor; // Get the right part

                *stone = left;
                right_items.push(right);
            } else {
                *stone *= 2024
            }
        }
        stones.extend(right_items);
    }
}

fn recursive_solution(stones: &Vec<u64>, number_of_blinks: u8) -> u64 {
    let mut answer = 0;
    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();
    for stone in stones {
        answer += count_stones_recursive(*stone, number_of_blinks, &mut cache);
    }
    answer
}

fn count_stones_recursive(value: u64, depth: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if let Some(&result) = cache.get(&(value, depth)) {
        return result;
    }
    if depth == 0 {
        return 1;
    }

    let result = if value == 0 {
        count_stones_recursive(1, depth - 1, cache)
    } else {
        let num_digits = (value as f64).log10().floor() as u32 + 1;
        if num_digits % 2 == 0 {
            let divisor = 10_u64.pow(num_digits / 2);
            let left = value / divisor;
            let right = value % divisor;

            count_stones_recursive(left, depth - 1, cache)
                + count_stones_recursive(right, depth - 1, cache)
        } else {
            count_stones_recursive(value * 2024, depth - 1, cache)
        }
    };

    cache.insert((value, depth), result);
    result
}
