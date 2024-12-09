use adv_code_2024::board_matrix::{Board, Point};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut text_matrix = Board::from_buffer(reader);
        let matrix_size = text_matrix.n_rows * text_matrix.n_cols;
        println!("Matrix size is {}", matrix_size);

        let (mut current_position, mut direction) =
            find_starting_position_and_direction(&text_matrix)?;

        text_matrix.set_value(
            current_position.y as usize,
            current_position.x as usize,
            'X',
        )?;

        let mut x_counter = 1;
        let mut n_steps = 0;

        loop {
            if n_steps > 5 * x_counter {
                panic!("You are running the loop too long!")
            }
            let next_position = current_position.add(&direction);

            match text_matrix.get_value_from_point(&next_position) {
                Err(_) => break, // It went outside the board
                Result::Ok(value) => match value {
                    '#' => {
                        direction = turn_direction_right(&direction);
                    }
                    'X' => {
                        current_position = next_position;
                        n_steps += 1;
                    }
                    '.' => {
                        current_position = next_position;
                        x_counter += 1;
                        text_matrix.set_value(
                            current_position.y as usize,
                            current_position.x as usize,
                            'X',
                        )?;
                        n_steps += 1;
                    }
                    _ => panic!("Incorrect value: {}!", value),
                },
            }
        }

        Ok(x_counter)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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

fn find_starting_position_and_direction(board: &Board<char>) -> Result<(Point, Point)> {
    for row in 0..board.n_rows {
        for col in 0..board.n_cols {
            match board.get_value(row, col)? {
                '>' => {
                    let direction = Point { x: 1, y: 0 };
                    let position = Point {
                        x: col as i32,
                        y: row as i32,
                    };
                    return Ok((position, direction));
                }
                '<' => {
                    let direction = Point { x: -1, y: 0 };
                    let position = Point {
                        x: col as i32,

                        y: row as i32,
                    };
                    return Ok((position, direction));
                }
                '^' => {
                    let direction = Point { x: 0, y: -1 };
                    let position = Point {
                        x: col as i32,
                        y: row as i32,
                    };
                    return Ok((position, direction));
                }
                'v' => {
                    let direction = Point { x: 0, y: 1 };
                    let position = Point {
                        x: col as i32,
                        y: row as i32,
                    };
                    return Ok((position, direction));
                }
                _ => continue,
            }
        }
    }
    Err(anyhow!("No starting position found!"))
}

fn turn_direction_right(direction: &Point) -> Point {
    Point {
        x: -direction.y,
        y: direction.x,
    }
}
