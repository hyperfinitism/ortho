// SPDX-License-Identifier: Apache-2.0

use crate::types::{Axis, Cell};

pub struct Board {
    size: usize,
    win_length: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(size: usize, win_length: usize) -> Self {
        Board {
            size,
            win_length,
            cells: vec![vec![Cell::Empty; size]; size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    pub fn check_win(&self, row: usize, col: usize, player: Cell) -> bool {
        let directions: [(isize, isize); 4] = [
            (0, 1),  // horizontal
            (1, 0),  // vertical
            (1, 1),  // diagonal ↘
            (1, -1), // diagonal ↙
        ];

        for (dr, dc) in &directions {
            let count = 1
                + self.count_consecutive(row, col, *dr, *dc, player)
                + self.count_consecutive(row, col, -dr, -dc, player);
            if count >= self.win_length {
                return true;
            }
        }
        false
    }

    fn count_consecutive(
        &self,
        row: usize,
        col: usize,
        dr: isize,
        dc: isize,
        player: Cell,
    ) -> usize {
        let mut count = 0;
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        while r >= 0
            && r < self.size as isize
            && c >= 0
            && c < self.size as isize
            && self.cells[r as usize][c as usize] == player
        {
            count += 1;
            r += dr;
            c += dc;
        }
        count
    }

    pub fn has_empty_along(&self, axis: Axis, index: usize) -> bool {
        match axis {
            Axis::Row => (0..self.size).any(|c| self.cells[index][c] == Cell::Empty),
            Axis::Col => (0..self.size).any(|r| self.cells[r][index] == Cell::Empty),
        }
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|&c| c != Cell::Empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new(5, 4);
        for r in 0..5 {
            for c in 0..5 {
                assert_eq!(board.get(r, c), Cell::Empty);
            }
        }
    }

    #[test]
    fn test_set_and_get() {
        let mut board = Board::new(5, 4);
        board.set(2, 3, Cell::Player1);
        assert_eq!(board.get(2, 3), Cell::Player1);
        assert_eq!(board.get(0, 0), Cell::Empty);
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::new(5, 4);
        for c in 0..4 {
            board.set(2, c, Cell::Player1);
        }
        assert!(board.check_win(2, 3, Cell::Player1));
        assert!(board.check_win(2, 0, Cell::Player1));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new(5, 4);
        for r in 1..5 {
            board.set(r, 0, Cell::Player2);
        }
        assert!(board.check_win(1, 0, Cell::Player2));
        assert!(board.check_win(4, 0, Cell::Player2));
    }

    #[test]
    fn test_diagonal_win() {
        let mut board = Board::new(5, 3);
        board.set(0, 0, Cell::Player1);
        board.set(1, 1, Cell::Player1);
        board.set(2, 2, Cell::Player1);
        assert!(board.check_win(1, 1, Cell::Player1));
    }

    #[test]
    fn test_anti_diagonal_win() {
        let mut board = Board::new(5, 3);
        board.set(0, 4, Cell::Player1);
        board.set(1, 3, Cell::Player1);
        board.set(2, 2, Cell::Player1);
        assert!(board.check_win(1, 3, Cell::Player1));
    }

    #[test]
    fn test_no_win() {
        let mut board = Board::new(5, 4);
        board.set(0, 0, Cell::Player1);
        board.set(0, 1, Cell::Player1);
        board.set(0, 2, Cell::Player1);
        // only 3, need 4
        assert!(!board.check_win(0, 2, Cell::Player1));
    }

    #[test]
    fn test_is_full() {
        let mut board = Board::new(2, 2);
        assert!(!board.is_full());
        board.set(0, 0, Cell::Player1);
        board.set(0, 1, Cell::Player2);
        board.set(1, 0, Cell::Player2);
        board.set(1, 1, Cell::Player1);
        assert!(board.is_full());
    }

    #[test]
    fn test_win_at_boundary() {
        let mut board = Board::new(4, 4);
        for r in 0..4 {
            board.set(r, 3, Cell::Player1);
        }
        assert!(board.check_win(0, 3, Cell::Player1));
        assert!(board.check_win(3, 3, Cell::Player1));
    }
}
