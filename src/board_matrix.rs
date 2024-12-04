use std::io::BufRead;
use anyhow::*;


pub struct Point {
    pub x: i32,
    pub y: i32,
}

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

    pub fn from_buffer<R: BufRead>(reader: R) -> Board<char> {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                rows.push(trimmed.chars().collect());
            }
        }
        Board::new(rows)
    }

    pub fn get_value(&self, row: usize, col: usize) -> Result<&T> {
        if row < self.n_rows && col < self.n_cols {
            return Ok(&self.board[row][col])
        }
        Err(Error::msg("Attempting to get out of bounds value"))
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

        let board: Board<char> = Board::<char>::from_buffer(input_buffer.as_bytes());
        assert_eq!(board.n_rows, 2);
        assert_eq!(board.n_cols, 5);
    }

    #[test]
    fn board_from_buffer_doesnt_read_empty_lines() {
        let input_buffer: &str = "\
            ABCDE

        ";

        let board: Board<char> = Board::<char>::from_buffer(input_buffer.as_bytes());
        assert_eq!(board.n_rows, 1);
    }
}