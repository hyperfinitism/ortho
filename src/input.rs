// SPDX-License-Identifier: Apache-2.0

use crate::types::{Axis, Selection};

pub enum PlayerInput {
    Select(Selection),
    Quit,
}

pub fn parse_input(input: &str, board_size: usize) -> Result<PlayerInput, String> {
    let input = input.trim().to_lowercase();

    if input == "q" || input == "quit" {
        return Ok(PlayerInput::Quit);
    }

    let (axis, num_str) = if let Some(rest) = input.strip_prefix("row") {
        (Axis::Row, rest.trim())
    } else if let Some(rest) = input.strip_prefix("col") {
        (Axis::Col, rest.trim())
    } else if let Some(rest) = input.strip_prefix('r') {
        (Axis::Row, rest.trim())
    } else if let Some(rest) = input.strip_prefix('c') {
        (Axis::Col, rest.trim())
    } else {
        return Err("Invalid format. Examples: r3, c2, row 3, col 2".to_string());
    };

    let number: usize = num_str
        .parse()
        .map_err(|_| format!("Expected a number, got: '{}'", num_str))?;

    if number == 0 || number > board_size {
        return Err(format!("Must be in range 1-{}", board_size));
    }

    Ok(PlayerInput::Select(Selection {
        axis,
        index: number - 1, // convert to 0-based
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row_short() {
        match parse_input("r3", 5).unwrap() {
            PlayerInput::Select(sel) => {
                assert_eq!(sel.axis, Axis::Row);
                assert_eq!(sel.index, 2); // 0-based
            }
            _ => panic!("Expected Select"),
        }
    }

    #[test]
    fn test_parse_col_long() {
        match parse_input("col 2", 5).unwrap() {
            PlayerInput::Select(sel) => {
                assert_eq!(sel.axis, Axis::Col);
                assert_eq!(sel.index, 1);
            }
            _ => panic!("Expected Select"),
        }
    }

    #[test]
    fn test_parse_row_long() {
        match parse_input("row 5", 5).unwrap() {
            PlayerInput::Select(sel) => {
                assert_eq!(sel.axis, Axis::Row);
                assert_eq!(sel.index, 4);
            }
            _ => panic!("Expected Select"),
        }
    }

    #[test]
    fn test_parse_quit() {
        assert!(matches!(parse_input("q", 5).unwrap(), PlayerInput::Quit));
        assert!(matches!(parse_input("quit", 5).unwrap(), PlayerInput::Quit));
        assert!(matches!(parse_input("QUIT", 5).unwrap(), PlayerInput::Quit));
    }

    #[test]
    fn test_parse_out_of_range() {
        assert!(parse_input("r0", 5).is_err());
        assert!(parse_input("r6", 5).is_err());
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_input("x3", 5).is_err());
        assert!(parse_input("", 5).is_err());
        assert!(parse_input("row abc", 5).is_err());
    }

    #[test]
    fn test_parse_with_whitespace() {
        match parse_input("  c 4  ", 5).unwrap() {
            PlayerInput::Select(sel) => {
                assert_eq!(sel.axis, Axis::Col);
                assert_eq!(sel.index, 3);
            }
            _ => panic!("Expected Select"),
        }
    }
}
