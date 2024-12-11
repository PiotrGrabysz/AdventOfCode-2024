use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let number_of_blinks = 25;
        let mut stones: Vec<usize> = reader
            .lines()
            .flatten()
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        for _ in 0..number_of_blinks {
            let mut right_items = Vec::new();
            for stone in &mut stones {
                if *stone == 0 {
                    *stone = 1;
                    continue;
                }
                let num_digits = (*stone as f64).log10().floor() as u32 + 1; // Calculate number of digits
                if num_digits % 2 == 0 {
                    let divisor = 10_usize.pow(num_digits / 2); // Find the split divisor
                    let left = *stone / divisor; // Get the left part
                    let right = *stone % divisor; // Get the right part

                    *stone = left;
                    right_items.push(right);
                }
                else {
                    *stone *= 2024
                }
            }
            stones.extend(right_items);
        }

        let answer = stones.len();

        Ok(answer)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}