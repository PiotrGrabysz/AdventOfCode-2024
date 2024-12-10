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
                let trail_ends = traverse_trail(
                    Point {
                        x: col as i32,
                        y: row as i32,
                    },
                    -1,
                    &topographic_map,
                );
                counter += count_unique_trail_ends(trail_ends);
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
) -> Vec<Point> {
    let current_height = match topographic_map.get_value_from_point(&current_position) {
        Result::Ok(value) => value,
        Err(_) => return Vec::new(),
    };

    if (current_height - previous_height) != 1 {
        return Vec::new();
    }
    if *current_height == 9 {
        return vec![current_position];
    }
    let mut trail_ends: Vec<Point> = vec![];
    for direction in &[
        Point { x: 1, y: 0 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
    ] {
        let next_position = current_position.add(direction);
        let trail_end = traverse_trail(next_position, *current_height, topographic_map);
        trail_ends.extend(trail_end);
    }
    trail_ends
}

fn count_unique_trail_ends(trail_ends: Vec<Point>) -> usize {
    let mut unique_points = HashSet::new();
    let mut counter = 0;
    for point in &trail_ends {
        if !unique_points.contains(point) {
            counter += 1;
            unique_points.insert(point);
        }
    }
    counter
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
    for direction in &[
        Point { x: 1, y: 0 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
    ] {
        let next_position = current_position.add(direction);
        total += traverse_trail_and_count_rating(next_position, *current_height, topographic_map);
    }
    total
}
