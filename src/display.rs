// SPDX-License-Identifier: Apache-2.0

use crate::board::Board;
use crate::types::{Axis, Selection};

/// Selections currently held by each player, used for display markers.
pub struct Selections {
    pub p1: Option<Selection>,
    pub p2: Option<Selection>,
}

pub fn render_board(board: &Board, selections: &Selections) {
    let size = board.size();
    let num_w = digit_count(size);
    // Left margin: 3-char row marker + space + row number + space
    let left = 3 + 1 + num_w + 1;

    // Column markers
    print!("{:left$}", "");
    for c in 0..size {
        let p1 = matches!(selections.p1, Some(Selection { axis: Axis::Col, index }) if index == c);
        let p2 = matches!(selections.p2, Some(Selection { axis: Axis::Col, index }) if index == c);
        let marker = match (p1, p2) {
            (true, true) => "X,O",
            (true, false) => "X",
            (false, true) => "O",
            (false, false) => "",
        };
        print!(" {:^3}", marker);
    }
    println!();

    // Column numbers
    print!("{:left$}", "");
    for c in 1..=size {
        print!(" {:^3}", c);
    }
    println!();

    let sep = format!("{:left$}+{}", "", ("---+").repeat(size));

    for r in 0..size {
        println!("{}", sep);

        let p1 = matches!(selections.p1, Some(Selection { axis: Axis::Row, index }) if index == r);
        let p2 = matches!(selections.p2, Some(Selection { axis: Axis::Row, index }) if index == r);
        let marker = match (p1, p2) {
            (true, true) => "X,O",
            (true, false) => "  X",
            (false, true) => "  O",
            (false, false) => "   ",
        };
        print!("{} {:>num_w$} |", marker, r + 1);
        for c in 0..size {
            print!(" {} |", board.get(r, c));
        }
        println!();
    }
    println!("{}", sep);
}

fn digit_count(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut val = n;
    while val > 0 {
        count += 1;
        val /= 10;
    }
    count
}
