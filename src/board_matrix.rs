use std::io::BufRead;
use anyhow::*;


#[derive(Debug)]
pub struct Board<T> {
    board: Vec<Vec<T>>,
    pub n_rows: usize,
    pub n_cols: usize,
}

impl<T> Board<T> {
    pub fn new(board: Vec<Vec<T>>) -> Self {
        let n_rows = board.len();
        let n_cols = board[0].len();

        Board {board, n_rows, n_cols}
    }

    pub fn get_value(&self, row: usize, col: usize) -> Result<&T> {
        if row < self.n_rows && col < self.n_cols {
            return Ok(&self.board[row][col])
        }
        Err(Error::msg("Attempting to get out of bounds value"))
    }

    fn get_value_from_point(&self, point: &Point) -> Result<&T> {
        self.get_value(point.y as usize, point.x as usize)
    }
}

impl Board<char> {
    pub fn from_buffer<R: BufRead>(reader: R) -> Self {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                rows.push(trimmed.chars().collect());
            }
        }
        Board::new(rows)
    }
}

enum Move {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Move {
    fn coordinates(&self) -> Point {
        match self {
            Move::Left => Point { x: -1, y: 0 },
            Move::Right => Point { x: 1, y: 0 },
            Move::Top => Point { x: 0, y: -1 },
            Move::Bottom => Point { x: 0, y: 1 },
            Move::TopLeft => Point { x: -1, y: -1 },
            Move::TopRight => Point { x: 1, y: -1 },
            Move::BottomLeft => Point { x: -1, y: 1 },
            Move::BottomRight => Point { x: 1, y: 1 },
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn add(&self, point: &Point) -> Point {
        Point {x: self.x + point.x, y: self.y + point.y}
    }
}

pub fn move_to_direction<'a, T>(
    board: &'a Board<T>,
    current_position: &Point,
    direction: Move,
) -> Result<(Point, &'a T)> {
    let move_coordinates = direction.coordinates();
    let new_position = current_position.add(&move_coordinates);

    let value = board.get_value_from_point(&new_position);

    match value {
        Result::Ok(value) => Ok((new_position,&value)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_initialize_board() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];

        let _ = Board::new(vec![row_1, row_2]);
    }

    #[test]
    fn has_correct_n_rows() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];

        let board = Board::new(vec![row_1, row_2]);
        assert_eq!(board.n_rows, 2);
    }

    #[test]
    fn has_correct_n_cols() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];

        let board = Board::new(vec![row_1, row_2]);
        assert_eq!(board.n_cols, 3);
    }

    #[test]
    fn read_value() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];
        let board = Board::new(vec![row_1, row_2]);

        let value = board.get_value(1, 2).unwrap();
        assert_eq!(*value, 6);
    }

    #[test]
    fn attempt_getting_out_of_bounds_index() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];
        let board = Board::new(vec![row_1, row_2]);

        let value = board.get_value(2, 0);
        assert!(matches!(value, Err(_)));
    }

    #[test]
    fn create_board_from_buffer() {
        let input_buffer: &str = "\
            ABCDE
            EFGHI";

        let board: Board<char> = Board::from_buffer(input_buffer.as_bytes());
        assert_eq!(board.n_rows, 2);
        assert_eq!(board.n_cols, 5);
    }

    #[test]
    fn board_from_buffer_doesnt_read_empty_lines() {
        let input_buffer: &str = "\
            ABCDE

        ";

        let board: Board<char> = Board::from_buffer(input_buffer.as_bytes());
        assert_eq!(board.n_rows, 1);
    }

    #[test]
    fn read_value_from_point() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];
        let board = Board::new(vec![row_1, row_2]);

        let point = Point { x: 2, y: 1 };

        let value = board.get_value_from_point(&point).unwrap();
        assert_eq!(*value, 6);
    }

    // Tests of moving to direction
    #[test]
    fn move_one_step_bottom_right() {
        let row_1 = vec![1, 2, 3];
        let row_2 = vec![4, 5, 6];
        let row_3 = vec![7, 8, 9];
        let board = Board::new(vec![row_1, row_2, row_3]);

        let (new_position, value) = move_to_direction(
            &board, &Point { x: 1, y: 1 }, Move::BottomRight,
        ).unwrap();

        assert_eq!(*value, 9);
        assert_eq!(new_position.x, 2);
        assert_eq!(new_position.y, 2);
    }
}