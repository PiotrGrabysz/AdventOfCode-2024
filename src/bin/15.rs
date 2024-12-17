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

#[derive(Copy, Clone)]
enum FieldType {
    Barrier,
    Obstacle,
    Empty,
    Robot,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: &mut R) -> Result<usize> {
        let (mut warehouse_map, moves) = read_map_and_moves(reader);

        let mut current_position = find_initial_position(&warehouse_map).unwrap();
        remove_initial_position_character(&mut warehouse_map, &current_position);

        let char_to_direction = get_char_to_direction_map();

        for move_char in moves.iter() {
            let direction = char_to_direction.get(move_char).unwrap();
            let new_position = current_position.add(direction);
            let next_value = warehouse_map.get_value_from_point(&new_position)?;

            match next_value {
                FieldType::Empty => {
                    current_position = new_position;
                }
                FieldType::Obstacle => {
                    let (last_index, last_value) =
                        get_last_index_behind_boxes(&warehouse_map, &current_position, direction);
                    match last_value {
                        FieldType::Empty => {
                            current_position = new_position;
                            warehouse_map.set_value(
                                current_position.y as usize,
                                current_position.x as usize,
                                FieldType::Empty,
                            )?;
                            warehouse_map.set_value(
                                last_index.y as usize,
                                last_index.x as usize,
                                FieldType::Obstacle,
                            )?;
                        }
                        _ => {}
                    }
                }
                FieldType::Barrier => continue,
                FieldType::Robot => {
                    return Err(anyhow!(
                        "Robot character should be removed during initialization!"
                    ));
                }
            }
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

fn read_map_and_moves<R: BufRead>(reader: &mut R) -> (Board<FieldType>, Vec<char>) {
    let warehouse_map = read_map(reader);
    let moves = read_moves(reader);
    (warehouse_map, moves)
}

fn read_map<R: BufRead>(reader: &mut R) -> Board<FieldType> {
    fn _map_char_to_field_type(c: char) -> Result<FieldType> {
        if c == '#' {
            return Ok(FieldType::Barrier);
        }
        if c == 'O' {
            return Ok(FieldType::Obstacle);
        }
        if c == '.' {
            return Ok(FieldType::Empty);
        }
        if c == '@' {
            return Ok(FieldType::Robot);
        }
        Err(anyhow!("Found invalid character: {}", c))
    }

    let mut rows: Vec<Vec<FieldType>> = Vec::new();
    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        rows.push(
            trimmed
                .chars()
                .into_iter()
                .map(|c| _map_char_to_field_type(c).unwrap())
                .collect()
        );
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

fn find_initial_position(map: &Board<FieldType>) -> Option<Point> {
    for row in 0..map.n_rows {
        for col in 0..map.n_cols {
            let value = map.get_value(row, col).unwrap();
            match value {
                FieldType::Robot => {
                    return Some(Point {
                        x: col as i32,
                        y: row as i32,
                    });
                }
                _ => {}
            }
        }
    }
    None
}

fn remove_initial_position_character(map: &mut Board<FieldType>, initial_position: &Point) {
    map.set_value(
        initial_position.y as usize,
        initial_position.x as usize,
        FieldType::Empty,
    )
    .unwrap()
}

fn get_char_to_direction_map() -> HashMap<char, Point> {
    let char_to_direction: HashMap<char, Point> = HashMap::from([
        ('<', Move::Left.coordinates()),
        ('>', Move::Right.coordinates()),
        ('^', Move::Top.coordinates()),
        ('v', Move::Bottom.coordinates()),
    ]);
    char_to_direction
}

fn get_last_index_behind_boxes(
    map: &Board<FieldType>,
    initial_position: &Point,
    direction: &Point,
) -> (Point, FieldType) {
    let mut new_position = initial_position.add(direction);
    let mut next_value = map
        .get_value_from_point(&new_position)
        .expect("The map is restricted by barriers '#', so I can't go outside of the map");

    let mut last_obstacle_in_row_found = false;
    while !last_obstacle_in_row_found {
        new_position = new_position.add(direction);
        next_value = map
            .get_value_from_point(&new_position)
            .expect("The map is restricted by barriers '#', so I can't go outside of the map");
        match next_value {
            FieldType::Obstacle => continue,
            _ => last_obstacle_in_row_found = true,
        }
    }
    (new_position, next_value.clone())
}

fn calculate_gps_score(map: &Board<FieldType>) -> usize {
    let mut score = 0;
    for row in 0..map.n_rows {
        for col in 0..map.n_cols {
            let value = map.get_value(row, col).unwrap();
            match value {
                FieldType::Obstacle => score += 100 * row + col,
                _ => continue,
            }
        }
    }
    score
}
