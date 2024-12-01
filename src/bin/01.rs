use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
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
        left_list.sort();
        right_list.sort();

        let mut difference: u32 = 0;
        for (x, y) in left_list.iter().zip(right_list) {
            difference += x.abs_diff(y)
        }
        Ok(difference)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

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
