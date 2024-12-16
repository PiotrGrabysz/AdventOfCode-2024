use adv_code_2024::board_matrix::{Board, Move, Point};
use adv_code_2024::start_day;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

const TEST_2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: &mut R) -> Result<usize> {
        let (mut warehouse_map, moves) = read_map_and_moves(reader);

        let mut current_position = find_initial_position(&warehouse_map).unwrap();
        remove_initial_position_character(&mut warehouse_map, &current_position);

        let char_to_direction: HashMap<char, Point> = HashMap::from([
            ('<', Move::Left.coordinates()),
            ('>', Move::Right.coordinates()),
            ('^', Move::Top.coordinates()),
            ('v', Move::Bottom.coordinates()),
        ]);

        // let initial_barrier_count = _counter_characters(&warehouse_map, '#');
        // let initial_box_count = _counter_characters(&warehouse_map, 'O');

        println!("Starting new map.");

        for move_char in moves.iter() {
            // println!("\nMoving {move_char:?}");
            let direction = char_to_direction.get(move_char).unwrap();
            let new_position = current_position.add(direction);
            let next_value = warehouse_map.get_value_from_point(&new_position)?;

            if *next_value == '.' {
                // warehouse_map.set_value(
                //     current_position.y as usize,
                //     current_position.x as usize,
                //     '.',
                // )?;
                // warehouse_map.set_value(new_position.y as usize, new_position.x as usize, '@')?;
                current_position = new_position;
            } else if *next_value == 'O' {
                let (last_index, last_value) =
                    get_last_index_behind_boxes(&warehouse_map, &current_position, direction);
                if last_value == '.' {
                    current_position = new_position;
                    warehouse_map.set_value(
                        current_position.y as usize,
                        current_position.x as usize,
                        '.',
                    )?;
                    // warehouse_map.set_value(
                    //     current_position.y as usize,
                    //     current_position.x as usize,
                    //     '@',
                    // )?;
                    warehouse_map.set_value(last_index.y as usize, last_index.x as usize, 'O')?;
                }
            } else if *next_value != '#' {
                return Err(anyhow!("Found invalid character: {}", *next_value));
            }
            // let barrier_count = _counter_characters(&warehouse_map, '#');
            // assert_eq!(initial_barrier_count, barrier_count);
            // let box_count = _counter_characters(&warehouse_map, 'O');
            // assert_eq!(initial_box_count, box_count);
            // let position_char_count = _counter_characters(&warehouse_map, '@');
            // assert_eq!(position_char_count, 1);
            // print_board(&warehouse_map);
        }

        Ok(calculate_gps_score(&warehouse_map))
    }

    assert_eq!(2028, part1(&mut BufReader::new(TEST.as_bytes()))?);
    assert_eq!(10092, part1(&mut BufReader::new(TEST_2.as_bytes()))?);
    println!("Two tests has passed.");

    let input_file = &mut BufReader::new(File::open(INPUT_FILE)?);
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

fn read_map_and_moves<R: BufRead>(reader: &mut R) -> (Board<char>, Vec<char>) {
    let warehouse_map = read_map(reader);
    let moves = read_moves(reader);
    (warehouse_map, moves)
}

fn read_map<R: BufRead>(reader: &mut R) -> Board<char> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        rows.push(trimmed.chars().collect());
    }
    Board::new(rows)
}

fn read_moves<R: BufRead>(reader: &mut R) -> Vec<char> {
    let mut moves: Vec<char> = vec![];
    let lines = reader.lines().flatten();
    for line in lines {
        let mut tmp: Vec<char> = line.chars().collect();
        moves.append(&mut tmp);
    }
    moves
}

fn find_initial_position(map: &Board<char>) -> Option<Point> {
    for row in 0..map.n_rows {
        for col in 0..map.n_cols {
            let value = map.get_value(row, col).unwrap();
            if *value == '@' {
                return Some(Point {
                    x: col as i32,
                    y: row as i32,
                });
            }
        }
    }
    None
}

fn remove_initial_position_character(map: &mut Board<char>, initial_position: &Point) {
    map.set_value(
        initial_position.y as usize,
        initial_position.x as usize,
        '.',
    )
    .unwrap()
}

fn get_last_index_behind_boxes(
    map: &Board<char>,
    initial_position: &Point,
    direction: &Point,
) -> (Point, char) {
    let mut new_position = initial_position.add(direction);
    let mut next_value = *map
        .get_value_from_point(&new_position)
        .expect("The map is restricted by barriers '#', so I can't go outside of the map");

    while next_value == 'O' {
        new_position = new_position.add(direction);
        next_value = *map
            .get_value_from_point(&new_position)
            .expect("The map is restricted by barriers '#', so I can't go outside of the map");
    }
    (new_position, next_value)
}

fn calculate_gps_score(map: &Board<char>) -> usize {
    let mut score = 0;
    for row in 0..map.n_rows {
        for col in 0..map.n_cols {
            let value = map.get_value(row, col).unwrap();
            if *value == 'O' {
                score += 100 * row + col;
            }
        }
    }
    score
}
//
// fn print_board(board: &Board<char>) {
//     println!();
//     for row in board.board.iter() {
//         let row: String = row.iter().collect();
//         println!("{}", row);
//     }
// }
//
// fn _counter_characters(map: &Board<char>, character: char) -> usize {
//     let mut count = 0;
//     for row in 0..map.n_rows {
//         for col in 0..map.n_cols {
//             let value = map.get_value(row, col).unwrap();
//             if *value == character {
//                 count += 1;
//             }
//         }
//     }
//     count
// }
