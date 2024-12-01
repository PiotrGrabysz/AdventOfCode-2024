use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let (mut left_list, mut right_list) = get_two_lists(reader)?;

        left_list.sort();
        right_list.sort();


        let mut difference: u32 = 0;
        for (x, y) in left_list.iter().zip(right_list) {
            difference += x.abs_diff(y)
        }
        Ok(difference)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let (left_list, right_list) = get_two_lists(reader)?;

        let mut right_items_counts = HashMap::new();
        for right_item in right_list {
            let count = right_items_counts.entry(right_item).or_insert(0);
            *count += 1;
        }

        let mut similarity_score: u32 = 0;
        for left_item in left_list {
            let count = right_items_counts.get(&left_item).unwrap_or(&0);
            similarity_score += count * left_item;

        }

        Ok(similarity_score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn get_two_lists<R: BufRead>(reader: R) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    let lines = reader.lines().flatten();
    for line in lines {
        let mut split_line = line.split_whitespace();
        let left_number = split_line.next().unwrap().parse::<u32>()?;
        let right_number = split_line.last().unwrap().parse::<u32>()?;

        left_list.push(left_number);
        right_list.push(right_number);
    }

    Ok((left_list, right_list))
}
