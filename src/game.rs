// SPDX-License-Identifier: Apache-2.0

use crate::board::Board;
use crate::player::PlayerState;
use crate::types::*;

pub struct Game {
    pub board: Board,
    pub players: [PlayerState; 2],
    pub current_player: usize,
}

impl Game {
    pub fn new(board_size: usize, win_length: usize) -> Self {
        Game {
            board: Board::new(board_size, win_length),
            players: [
                PlayerState::new(PlayerId::P1),
                PlayerState::new(PlayerId::P2),
            ],
            current_player: 0,
        }
    }

    pub fn current(&self) -> &PlayerState {
        &self.players[self.current_player]
    }

    pub fn process_turn(&mut self, selection: Selection) -> Result<TurnResult, GameError> {
        let size = self.board.size();

        // Bounds check
        if selection.index >= size {
            return Err(GameError::OutOfBounds {
                index: selection.index,
                max: size,
            });
        }

        // Alternation constraint
        if let Some(expected) = self.players[self.current_player].required_axis()
            && selection.axis != expected
        {
            return Err(GameError::WrongAxis { expected });
        }

        let player = &self.players[self.current_player];

        // First move (no pending selection): store selection, no stone.
        // Must pick a row/col with at least one empty cell.
        if player.last_selection.is_none() {
            if !self.board.has_empty_along(selection.axis, selection.index) {
                return Err(GameError::NoEmptyCell {
                    axis: selection.axis,
                    index: selection.index,
                });
            }
            self.players[self.current_player].last_selection = Some(selection);
            return Ok(TurnResult::FirstMove);
        }

        // Compute intersection
        let prev = player.last_selection.unwrap();
        let (row, col) = match selection.axis {
            Axis::Row => (selection.index, prev.index),
            Axis::Col => (prev.index, selection.index),
        };

        // Check if occupied — player must pick a different index
        let player_id = self.players[self.current_player].id;
        if self.board.get(row, col) != Cell::Empty {
            return Err(GameError::Occupied { row, col });
        }

        // Place stone — swap selection to the completing axis
        self.board.set(row, col, player_id.cell());
        self.players[self.current_player].last_selection = Some(selection);

        if self.board.check_win(row, col, player_id.cell()) {
            return Ok(TurnResult::Win { row, col });
        }

        Ok(TurnResult::StonePlaced { row, col })
    }

    pub fn check_skip(&mut self) -> Option<TurnResult> {
        if let Some(sel) = self.players[self.current_player].last_selection
            && !self.board.has_empty_along(sel.axis, sel.index)
        {
            self.players[self.current_player].clear_selection();
            return Some(TurnResult::Skipped);
        }
        None
    }

    pub fn switch_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_move_no_stone() {
        let mut game = Game::new(5, 4);
        let sel = Selection {
            axis: Axis::Col,
            index: 2,
        };
        let result = game.process_turn(sel).unwrap();
        assert!(matches!(result, TurnResult::FirstMove));
    }

    #[test]
    fn test_completing_move_swaps_selection() {
        let mut game = Game::new(5, 4);
        // P1 first move: col 2
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 2,
        })
        .unwrap();
        game.switch_player();
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 0,
        })
        .unwrap();
        game.switch_player();
        // P1 completing move: row 3 -> stone at (3, 2)
        let result = game
            .process_turn(Selection {
                axis: Axis::Row,
                index: 3,
            })
            .unwrap();
        match result {
            TurnResult::StonePlaced { row, col } => {
                assert_eq!(row, 3);
                assert_eq!(col, 2);
            }
            _ => panic!("Expected StonePlaced"),
        }
        assert_eq!(game.board.get(3, 2), Cell::Player1);
        // Selection swapped: now locked on row 3, must pick col next
        let sel = game.players[0].last_selection.unwrap();
        assert_eq!(sel.axis, Axis::Row);
        assert_eq!(sel.index, 3);
        assert_eq!(game.players[0].required_axis(), Some(Axis::Col));
    }

    #[test]
    fn test_selection_swaps_across_turns() {
        let mut game = Game::new(5, 4);
        // P1: col 2
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 2,
        })
        .unwrap();
        game.switch_player();
        // P2: row 0
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 0,
        })
        .unwrap();
        game.switch_player();
        // P1: row 3 -> stone at (3, 2), selection swaps to row 3
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 3,
        })
        .unwrap();
        game.switch_player();
        // P2: col 1 -> stone at (0, 1), selection swaps to col 1
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 1,
        })
        .unwrap();
        game.switch_player();
        // P1 now locked on row 3, must pick col
        assert_eq!(game.players[0].required_axis(), Some(Axis::Col));
        let result = game
            .process_turn(Selection {
                axis: Axis::Col,
                index: 4,
            })
            .unwrap();
        assert!(matches!(result, TurnResult::StonePlaced { row: 3, col: 4 }));
        // Now selection swapped to col 4
        let sel = game.players[0].last_selection.unwrap();
        assert_eq!(sel.axis, Axis::Col);
        assert_eq!(sel.index, 4);
    }

    #[test]
    fn test_occupied_intersection_is_error() {
        let mut game = Game::new(5, 4);
        game.board.set(3, 2, Cell::Player2);

        // P1 first move: col 2
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 2,
        })
        .unwrap();
        game.switch_player();
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 0,
        })
        .unwrap();
        game.switch_player();

        // P1 completing move: row 3 -> (3,2) is occupied -> error
        let result = game.process_turn(Selection {
            axis: Axis::Row,
            index: 3,
        });
        assert!(matches!(
            result,
            Err(GameError::Occupied { row: 3, col: 2 })
        ));
        // Selection should NOT be cleared — player retries
        assert!(game.players[0].last_selection.is_some());
    }

    #[test]
    fn test_occupied_then_valid_pick() {
        let mut game = Game::new(5, 4);
        game.board.set(3, 2, Cell::Player2);

        // P1: col 2
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 2,
        })
        .unwrap();
        game.switch_player();
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 0,
        })
        .unwrap();
        game.switch_player();

        // Try row 3 -> occupied -> error
        assert!(
            game.process_turn(Selection {
                axis: Axis::Row,
                index: 3,
            })
            .is_err()
        );

        // Try row 1 -> empty -> should work
        let result = game
            .process_turn(Selection {
                axis: Axis::Row,
                index: 1,
            })
            .unwrap();
        assert!(matches!(result, TurnResult::StonePlaced { row: 1, col: 2 }));
    }

    #[test]
    fn test_auto_skip_clears_selection() {
        let mut game = Game::new(2, 2);
        // Fill column 0 entirely
        game.board.set(0, 0, Cell::Player1);
        game.board.set(1, 0, Cell::Player2);

        // Manually set P1's selection to col 0
        game.players[0].last_selection = Some(Selection {
            axis: Axis::Col,
            index: 0,
        });

        // Col 0 is full -> auto-skip
        let result = game.check_skip();
        assert!(matches!(result, Some(TurnResult::Skipped)));
        assert!(game.players[0].last_selection.is_none());
    }

    #[test]
    fn test_no_skip_when_valid_completion_exists() {
        let mut game = Game::new(5, 4);
        // P1 selects col 2
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 2,
        })
        .unwrap();
        game.switch_player();
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 0,
        })
        .unwrap();
        game.switch_player();

        // Col 2 has empty cells -> no skip
        let result = game.check_skip();
        assert!(result.is_none());
    }

    #[test]
    fn test_free_choice_after_skip() {
        let mut game = Game::new(2, 2);
        game.board.set(0, 0, Cell::Player1);
        game.board.set(1, 0, Cell::Player2);

        // P1 had col 0, gets skipped
        game.players[0].last_selection = Some(Selection {
            axis: Axis::Col,
            index: 0,
        });
        game.check_skip(); // clears selection

        // Now P1 has free choice
        assert!(game.players[0].required_axis().is_none());
        let result = game
            .process_turn(Selection {
                axis: Axis::Col,
                index: 1,
            })
            .unwrap();
        assert!(matches!(result, TurnResult::FirstMove));
    }

    #[test]
    fn test_free_selection_rejects_full_row() {
        let mut game = Game::new(2, 2);
        // Fill row 0
        game.board.set(0, 0, Cell::Player1);
        game.board.set(0, 1, Cell::Player2);

        // P1 tries to select row 0 (no empty cells) -> error
        let result = game.process_turn(Selection {
            axis: Axis::Row,
            index: 0,
        });
        assert!(matches!(
            result,
            Err(GameError::NoEmptyCell {
                axis: Axis::Row,
                index: 0
            })
        ));
    }

    #[test]
    fn test_free_selection_accepts_row_with_empty() {
        let mut game = Game::new(5, 4);
        // Row 1 has empty cells
        let result = game
            .process_turn(Selection {
                axis: Axis::Row,
                index: 1,
            })
            .unwrap();
        assert!(matches!(result, TurnResult::FirstMove));
    }

    #[test]
    fn test_wrong_axis_error() {
        let mut game = Game::new(5, 4);
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 2,
        })
        .unwrap();
        game.switch_player();
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 0,
        })
        .unwrap();
        game.switch_player();
        // Try col again (should be row)
        let result = game.process_turn(Selection {
            axis: Axis::Col,
            index: 1,
        });
        assert!(matches!(
            result,
            Err(GameError::WrongAxis {
                expected: Axis::Row
            })
        ));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut game = Game::new(5, 4);
        let result = game.process_turn(Selection {
            axis: Axis::Row,
            index: 5,
        });
        assert!(matches!(result, Err(GameError::OutOfBounds { .. })));
    }

    #[test]
    fn test_win_detection() {
        let mut game = Game::new(5, 3);
        game.board.set(0, 0, Cell::Player1);
        game.board.set(0, 1, Cell::Player1);

        // P1: col 2 -> first move
        game.process_turn(Selection {
            axis: Axis::Col,
            index: 2,
        })
        .unwrap();
        game.switch_player();
        game.process_turn(Selection {
            axis: Axis::Row,
            index: 4,
        })
        .unwrap();
        game.switch_player();

        // P1: row 0 -> stone at (0, 2) -> 3 in a row -> win
        let result = game
            .process_turn(Selection {
                axis: Axis::Row,
                index: 0,
            })
            .unwrap();
        assert!(matches!(result, TurnResult::Win { row: 0, col: 2 }));
    }
}
