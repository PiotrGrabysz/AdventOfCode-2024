use adv_code_2024::board_matrix::{Board, Move, MoveIterator, Point};
use adv_code_2024::start_day;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let text_matrix = Board::from_buffer(reader);

        let pattern: Vec<char> = "XMAS".chars().collect();

        let directions = vec![
            Move::Left,
            Move::Right,
            Move::Top,
            Move::Bottom,
            Move::TopLeft,
            Move::TopRight,
            Move::BottomLeft,
            Move::BottomRight,
        ];

        let mut counter = 0;
        for i in 0..text_matrix.n_cols {
            for j in 0..text_matrix.n_rows {
                let current_position = Point {
                    x: i as i32,
                    y: j as i32,
                };

                for direction in &directions {
                    let move_iterator =
                        MoveIterator::new(&text_matrix, &current_position, direction);

                    for ((idx, pattern_letter), matrix_letter) in
                        pattern.iter().enumerate().zip(move_iterator)
                    {
                        if pattern_letter != matrix_letter {
                            break;
                        }
                        if idx == 3 {
                            counter += 1
                        }
                    }
                }
            }
        }

        Ok(counter)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

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
