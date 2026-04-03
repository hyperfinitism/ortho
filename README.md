# ORTHO

![SemVer: pre-release](https://img.shields.io/badge/ortho-pre--release-blue)
![MSRV: 1.88.0](https://img.shields.io/badge/MSRV-1.88.0-brown.svg)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-red.svg)](https://www.apache.org/licenses/LICENSE-2.0)

**ORTHO** is a two-player CLI board game where players take turns selecting rows and columns on an N×N grid, place stones at the intersection of a player's current and previous selections, and race to make M-in-a-row.

## Rules

1. Two players alternate turns on an N×N board.
2. On each turn, a player selects either a **row** or a **column**.
3. On the **first selection** (no pending selection), a player may choose either axis freely, but must pick a row or column that has at least one empty cell.
   No stone is placed (only one coordinate is known).
4. On the **completing selection**, the player must choose the **opposite axis** (if they chose a column, they must choose a row, and vice versa). The intersection must be **empty** — if it is occupied, the player must pick a different index.
5. A stone is placed at the intersection. The completing selection then **replaces** the previous one — the player stays locked on the new axis for their next turn.
6. If the locked row/column has **no empty cells** remaining, the turn is **skipped** and the selection is **cleared**. On their next turn the player returns to a free choice.
7. The first player to get **M consecutive stones** in a row, column, or diagonal wins.
8. If the board is completely filled with no winner, the game is a draw.

The board display shows which row or column each player currently has selected (`X` for Player 1, `O` for Player 2) along the margins.

## Usage

```
ortho [OPTIONS]
```

### Options

| Flag | Description | Default |
| ---- | ----------- | ------- |
| `-n`, `--size <N>` | Board size (N×N) | 5 |
| `-m`, `--win-length <M>` | Consecutive stones needed to win (M ≤ N) | 4 |

### Examples

```sh
ortho               # 5×5 board, 4-in-a-row to win
ortho -n 7 -m 5     # 7×7 board, 5-in-a-row to win
ortho -n 3 -m 3     # 3×3 board, 3-in-a-row to win
```

### Input Format

| Input | Meaning |
|-------|---------|
| `r3` or `row 3` | Select row 3 |
| `c2` or `col 2` | Select column 2 |
| `q` or `quit` | Quit the game |

### Gameplay Example

```plaintext
=== Ortho ===  Board: 5x5  Win condition: 4 in a row
Input examples: r3 (row 3), c2 (col 2), quit (exit)


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 |   |   |   |   |   |
      +---+---+---+---+---+
    4 |   |   |   |   |   |
      +---+---+---+---+---+
    5 |   |   |   |   |   |
      +---+---+---+---+---+

Player 1 (X) - Choose any row or column > c3
Selected col 3 (first move, no stone placed)

                X
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 |   |   |   |   |   |
      +---+---+---+---+---+
    4 |   |   |   |   |   |
      +---+---+---+---+---+
    5 |   |   |   |   |   |
      +---+---+---+---+---+

Player 2 (O) - Choose any row or column > r3
Selected row 3 (first move, no stone placed)

                X
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
  O 3 |   |   |   |   |   |
      +---+---+---+---+---+
    4 |   |   |   |   |   |
      +---+---+---+---+---+
    5 |   |   |   |   |   |
      +---+---+---+---+---+

Player 1 (X) - Choose a row > r3
Placed stone at (3, 3)!


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
X,O 3 |   |   | X |   |   |
      +---+---+---+---+---+
    4 |   |   |   |   |   |
      +---+---+---+---+---+
    5 |   |   |   |   |   |
      +---+---+---+---+---+

Player 2 (O) - Choose a col > c2
Placed stone at (3, 2)!

            O
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
  X 3 |   | O | X |   |   |
      +---+---+---+---+---+
    4 |   |   |   |   |   |
      +---+---+---+---+---+
    5 |   |   |   |   |   |
      +---+---+---+---+---+

Player 1 (X) - Choose a col > c1
Placed stone at (3, 1)!

        X   O
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X |   |   |
      +---+---+---+---+---+
    4 |   |   |   |   |   |
      +---+---+---+---+---+
    5 |   |   |   |   |   |
      +---+---+---+---+---+

Player 2 (O) - Choose a row > r4
Placed stone at (4, 2)!

        X
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X |   |   |
      +---+---+---+---+---+
  O 4 |   | O |   |   |   |
      +---+---+---+---+---+
    5 |   |   |   |   |   |
      +---+---+---+---+---+

Player 1 (X) - Choose a row > r5
Placed stone at (5, 1)!


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X |   |   |
      +---+---+---+---+---+
  O 4 |   | O |   |   |   |
      +---+---+---+---+---+
  X 5 | X |   |   |   |   |
      +---+---+---+---+---+

Player 2 (O) - Choose a col > c4
Placed stone at (4, 4)!

                    O
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X |   |   |
      +---+---+---+---+---+
    4 |   | O |   | O |   |
      +---+---+---+---+---+
  X 5 | X |   |   |   |   |
      +---+---+---+---+---+

Player 1 (X) - Choose a col > c5
Placed stone at (5, 5)!

                    O   X
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X |   |   |
      +---+---+---+---+---+
    4 |   | O |   | O |   |
      +---+---+---+---+---+
    5 | X |   |   |   | X |
      +---+---+---+---+---+

Player 2 (O) - Choose a row > r3
Placed stone at (3, 4)!

                        X
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
  O 3 | X | O | X | O |   |
      +---+---+---+---+---+
    4 |   | O |   | O |   |
      +---+---+---+---+---+
    5 | X |   |   |   | X |
      +---+---+---+---+---+

Player 1 (X) - Choose a row > r3
Placed stone at (3, 5)!


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
X,O 3 | X | O | X | O | X |
      +---+---+---+---+---+
    4 |   | O |   | O |   |
      +---+---+---+---+---+
    5 | X |   |   |   | X |
      +---+---+---+---+---+

Player 2 (O) - No valid placement available. Turn skipped, selection cleared.


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
  X 3 | X | O | X | O | X |
      +---+---+---+---+---+
    4 |   | O |   | O |   |
      +---+---+---+---+---+
    5 | X |   |   |   | X |
      +---+---+---+---+---+

Player 1 (X) - No valid placement available. Turn skipped, selection cleared.


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X | O | X |
      +---+---+---+---+---+
    4 |   | O |   | O |   |
      +---+---+---+---+---+
    5 | X |   |   |   | X |
      +---+---+---+---+---+

Player 2 (O) - Choose any row or column > c4
Selected col 4 (first move, no stone placed)

                    O
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X | O | X |
      +---+---+---+---+---+
    4 |   | O |   | O |   |
      +---+---+---+---+---+
    5 | X |   |   |   | X |
      +---+---+---+---+---+

Player 1 (X) - Choose any row or column > r4
Selected row 4 (first move, no stone placed)

                    O
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X | O | X |
      +---+---+---+---+---+
  X 4 |   | O |   | O |   |
      +---+---+---+---+---+
    5 | X |   |   |   | X |
      +---+---+---+---+---+

Player 2 (O) - Choose a row > r5
Placed stone at (5, 4)!


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X | O | X |
      +---+---+---+---+---+
  X 4 |   | O |   | O |   |
      +---+---+---+---+---+
  O 5 | X |   |   | O | X |
      +---+---+---+---+---+

Player 1 (X) - Choose a col > c5
Placed stone at (4, 5)!

                        X
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X | O | X |
      +---+---+---+---+---+
    4 |   | O |   | O | X |
      +---+---+---+---+---+
  O 5 | X |   |   | O | X |
      +---+---+---+---+---+

Player 2 (O) - Choose a col > c3
Placed stone at (5, 3)!

                O       X
        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   |   |
      +---+---+---+---+---+
    3 | X | O | X | O | X |
      +---+---+---+---+---+
    4 |   | O |   | O | X |
      +---+---+---+---+---+
    5 | X |   | O | O | X |
      +---+---+---+---+---+

Player 1 (X) - Choose a row > r2
Placed stone at (2, 5)!


        1   2   3   4   5
      +---+---+---+---+---+
    1 |   |   |   |   |   |
      +---+---+---+---+---+
    2 |   |   |   |   | X |
      +---+---+---+---+---+
    3 | X | O | X | O | X |
      +---+---+---+---+---+
    4 |   | O |   | O | X |
      +---+---+---+---+---+
    5 | X |   | O | O | X |
      +---+---+---+---+---+

Player 1 (X) wins!
```

## Installation

```sh
cargo install --git https://github.com/hyperfinitism/ortho
```
