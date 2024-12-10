use adv_code_2024::board_matrix::{Board, Point};
use adv_code_2024::start_day;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

const DIRECTIONS_TO_MOVE: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let topographic_map = Board::<i8>::from_buffer(reader);

        let mut counter = 0;
        for row in 0..topographic_map.n_rows {
            for col in 0..topographic_map.n_cols {
                let value = topographic_map.get_value(row, col)?;
                if *value != 0 {
                    continue;
                }
                let mut unique_trail_ends: HashSet<Point> = HashSet::new();
                traverse_trail(
                    Point {
                        x: col as i32,
                        y: row as i32,
                    },
                    -1,
                    &topographic_map,
                    &mut unique_trail_ends,
                );
                counter += unique_trail_ends.len()
            }
        }
        Ok(counter)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let topographic_map = Board::<i8>::from_buffer(reader);

        let mut counter = 0;
        for row in 0..topographic_map.n_rows {
            for col in 0..topographic_map.n_cols {
                let value = topographic_map.get_value(row, col)?;
                if *value != 0 {
                    continue;
                }
                counter += traverse_trail_and_count_rating(
                    Point {
                        x: col as i32,
                        y: row as i32,
                    },
                    -1,
                    &topographic_map,
                );
            }
        }
        Ok(counter)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn traverse_trail(
    current_position: Point,
    previous_height: i8,
    topographic_map: &Board<i8>,
    unique_points: &mut HashSet<Point>,
) -> () {
    let current_height = match topographic_map.get_value_from_point(&current_position) {
        Result::Ok(value) => value,
        Err(_) => return,
    };

    if (current_height - previous_height) != 1 {
        return;
    }
    if *current_height == 9 {
        unique_points.insert(current_position);
        return;
    }

    for direction in &DIRECTIONS_TO_MOVE {
        let next_position = current_position.add(direction);
        let _ = traverse_trail(
            next_position,
            *current_height,
            topographic_map,
            unique_points,
        );
    }
}

fn traverse_trail_and_count_rating(
    current_position: Point,
    previous_height: i8,
    topographic_map: &Board<i8>,
) -> usize {
    let current_height = match topographic_map.get_value_from_point(&current_position) {
        Result::Ok(value) => value,
        Err(_) => return 0,
    };

    if (current_height - previous_height) != 1 {
        return 0;
    }
    if *current_height == 9 {
        return 1;
    }

    let mut total = 0;
    for direction in &DIRECTIONS_TO_MOVE {
        let next_position = current_position.add(direction);
        total += traverse_trail_and_count_rating(next_position, *current_height, topographic_map);
    }
    total
}
