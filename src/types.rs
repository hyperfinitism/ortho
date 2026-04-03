// SPDX-License-Identifier: Apache-2.0

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Row,
    Col,
}

impl Axis {
    pub fn opposite(self) -> Axis {
        match self {
            Axis::Row => Axis::Col,
            Axis::Col => Axis::Row,
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Axis::Row => write!(f, "row"),
            Axis::Col => write!(f, "col"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    pub axis: Axis,
    pub index: usize, // 0-based
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Player1,
    Player2,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Player1 => write!(f, "X"),
            Cell::Player2 => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    P1,
    P2,
}

impl PlayerId {
    pub fn cell(self) -> Cell {
        match self {
            PlayerId::P1 => Cell::Player1,
            PlayerId::P2 => Cell::Player2,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PlayerId::P1 => "Player 1 (X)",
            PlayerId::P2 => "Player 2 (O)",
        }
    }
}

#[derive(Debug)]
pub enum TurnResult {
    FirstMove,
    StonePlaced { row: usize, col: usize },
    Skipped,
    Win { row: usize, col: usize },
}

#[derive(Debug)]
pub enum GameError {
    OutOfBounds { index: usize, max: usize },
    WrongAxis { expected: Axis },
    Occupied { row: usize, col: usize },
    NoEmptyCell { axis: Axis, index: usize },
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::OutOfBounds { index, max } => {
                write!(f, "Out of bounds: {} (must be 1-{})", index + 1, max)
            }
            GameError::WrongAxis { expected } => {
                write!(f, "You must choose a {}", expected)
            }
            GameError::Occupied { row, col } => {
                write!(
                    f,
                    "({}, {}) is already occupied. Pick a different index",
                    row + 1,
                    col + 1,
                )
            }
            GameError::NoEmptyCell { axis, index } => {
                write!(
                    f,
                    "{} {} has no empty cells. Pick a different {}",
                    axis,
                    index + 1,
                    axis,
                )
            }
        }
    }
}
