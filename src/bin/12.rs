use adv_code_2024::board_matrix::{Board, Point};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
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
        let region_map: Board<char> = Board::<char>::from_buffer(reader);

        let mut total_price = 0;
        let mut already_visited: HashSet<Point> = HashSet::new();
        for row in 0..region_map.n_rows {
            for col in 0..region_map.n_cols {
                let current_point = Point {
                    x: col as i32,
                    y: row as i32,
                };
                let current_region_code = region_map.get_value_from_point(&current_point)?;
                let (area, perimeter) = traverse_region_map(
                    current_point,
                    current_region_code,
                    &region_map,
                    &mut already_visited,
                );
                total_price += perimeter * area;
            }
        }

        Ok(total_price)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

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

fn traverse_region_map(
    current_position: Point,
    previous_region_code: &char,
    region_map: &Board<char>,
    already_visited: &mut HashSet<Point>,
) -> (usize, usize) {
    let current_region_code = match region_map.get_value_from_point(&current_position) {
        Result::Ok(value) => value,
        Err(_) => return (0, 1),
    };

    if current_region_code != previous_region_code {
        return (0, 1);
    }

    if already_visited.contains(&current_position) {
        return (0, 0);
    }

    already_visited.insert(Point {
        x: current_position.x,
        y: current_position.y,
    });

    let mut area = 1;
    let mut perimeter = 0;
    for direction in &DIRECTIONS_TO_MOVE {
        let next_position = current_position.add(direction);
        let (partial_area, partial_perimeter) = traverse_region_map(
            next_position,
            current_region_code,
            region_map,
            already_visited,
        );
        area += partial_area;
        perimeter += partial_perimeter;
    }
    (area, perimeter)
}
