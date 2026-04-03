// SPDX-License-Identifier: Apache-2.0

mod board;
mod display;
mod game;
mod input;
mod player;
mod types;

use clap::Parser;
use std::io::{self, Write};

use crate::display::{Selections, render_board};
use crate::game::Game;
use crate::input::{PlayerInput, parse_input};
use crate::types::TurnResult;

#[derive(Parser)]
#[command(name = "ortho", about = "Orthogonal selection board game")]
struct Args {
    /// Board size (N×N)
    #[arg(short = 'n', long = "size", default_value_t = 5)]
    board_size: usize,

    /// Number of consecutive stones needed to win
    #[arg(short = 'm', long = "win-length", default_value_t = 4)]
    win_length: usize,
}

fn main() {
    let args = Args::parse();

    if args.board_size == 0 {
        eprintln!("Error: board size must be at least 1");
        std::process::exit(1);
    }
    if args.win_length == 0 || args.win_length > args.board_size {
        eprintln!("Error: win length must be in range 1-{}", args.board_size);
        std::process::exit(1);
    }

    println!(
        "=== Ortho ===  Board: {}x{}  Win condition: {} in a row",
        args.board_size, args.board_size, args.win_length
    );
    println!("Input examples: r3 (row 3), c2 (col 2), quit (exit)\n");

    let mut game = Game::new(args.board_size, args.win_length);

    loop {
        let sels = Selections {
            p1: game.players[0].last_selection,
            p2: game.players[1].last_selection,
        };
        render_board(&game.board, &sels);
        println!();

        // Auto-skip if no valid completion exists
        if let Some(TurnResult::Skipped) = game.check_skip() {
            println!(
                "{} - No valid placement available. Turn skipped, selection cleared.\n",
                game.current().id.name()
            );
            game.switch_player();
            continue;
        }

        let player = game.current();
        let constraint_msg = match player.required_axis() {
            Some(axis) => format!("Choose a {}", axis),
            None => "Choose any row or column".to_string(),
        };
        print!("{} - {} > ", player.id.name(), constraint_msg);
        io::stdout().flush().expect("failed to flush stdout");

        let mut input_line = String::new();
        if io::stdin()
            .read_line(&mut input_line)
            .expect("failed to read from stdin")
            == 0
        {
            println!("\nExiting game.");
            break;
        }

        let player_input = match parse_input(&input_line, args.board_size) {
            Ok(pi) => pi,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        match player_input {
            PlayerInput::Quit => {
                println!("Exiting game.");
                break;
            }
            PlayerInput::Select(selection) => {
                let axis_name = selection.axis;
                let display_index = selection.index + 1;

                match game.process_turn(selection) {
                    Ok(TurnResult::FirstMove) => {
                        println!(
                            "Selected {} {} (first move, no stone placed)\n",
                            axis_name, display_index
                        );
                    }
                    Ok(TurnResult::StonePlaced { row, col }) => {
                        println!("Placed stone at ({}, {})!\n", row + 1, col + 1);
                    }
                    Ok(TurnResult::Skipped) => unreachable!(),
                    Ok(TurnResult::Win { row, col }) => {
                        println!("Placed stone at ({}, {})!\n", row + 1, col + 1);
                        let sels = Selections { p1: None, p2: None };
                        render_board(&game.board, &sels);
                        println!("\n{} wins!", game.current().id.name());
                        return;
                    }
                    Err(e) => {
                        println!("Error: {}\n", e);
                        continue;
                    }
                }

                if game.board.is_full() {
                    let sels = Selections { p1: None, p2: None };
                    render_board(&game.board, &sels);
                    println!("\nDraw!");
                    return;
                }

                game.switch_player();
            }
        }
    }
}
