use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let disk_map: Vec<usize> = reader
            .lines()
            .flatten()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|num| num as usize)
            .collect();

        let mut blocks: Vec<i32> = Vec::new();
        let mut free_space_digit = false;
        let mut block_index = 0;

        for digit in disk_map {
            if free_space_digit {
                blocks.append(&mut vec![-1; digit]);
            }
            else {
                blocks.append(&mut vec![block_index; digit]);
                block_index += 1;
            }

            free_space_digit = !free_space_digit;
        }

        let mut left_index = 0;
        let mut right_index = blocks.len() - 1;
        while left_index < right_index {
            let digit = blocks[left_index];
            if digit == -1 {
                let mut digit_to_move = blocks[right_index];
                while digit_to_move == -1 {
                    right_index -= 1;
                    digit_to_move = blocks[right_index]
                }
                blocks[left_index] = digit_to_move;
                blocks[right_index] = 0;
                right_index -= 1;
            }
            left_index += 1;
        }

        let answer = calculate_hash(&blocks);
        Ok(answer)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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

fn calculate_hash(blocks: &Vec<i32>) -> i64 {
    let mut hash: i64  = 0;
    for (idx, digit) in blocks.iter().enumerate() {
        if *digit == -1 {
            break
        }
        hash += (idx as i64) * (*digit as i64);
    }
    hash
}