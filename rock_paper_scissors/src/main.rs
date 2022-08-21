pub use std::io;
mod GameMoves;
use crate::GameMoves::game_moves::{get_comp_move, Moves};

fn main() {
    let mut playing: bool = true;
    let mut round_ct: i32 = 1;
    let mut comp_move: Moves;
    let mut player_win: i32 = 0;
    let mut comp_win: i32 = 0;

    println!("---RPS---");
    while playing {
        println!("-Round {}-", round_ct);

        //moves
        let mut player_move: String = String::new(); //will shadow later
        println!("Enter move:");
        println!("1/R - Rock");
        println!("2/P - Paper");
        println!("3/S - Scissors");
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");
        let player_move: Moves = match player_move.trim() {
            "1" | "R" | "r" => Moves::Rock,
            "2" | "P" | "p" => Moves::Paper,
            "3" | "S" | "s" => Moves::Scissors,
            _ => {
                println!("Invalid input");
                println!("-Terminating Round-");
                continue;
            }
        };
        println!("You chose {player_move}");

        comp_move = get_comp_move();
        println!("Comp chose {comp_move}");

        //round outcome
        //enums are 0 based, can cast to i32 for comparison
        let p_move_int: i32 = player_move as i32;
        let c_move_int: i32 = comp_move as i32;
        if (p_move_int + 1) % 3 == c_move_int {
            println!("~Comp Wins!~");
            comp_win += 1;
        } else if p_move_int == c_move_int {
            println!("~DRAW~");
        } else {
            println!("~Player Wins!~");
            player_win += 1;
        }

        //New round
        let mut play_again: String = String::new();
        println!("\nNew round? (y/n): ");
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");
        playing = if play_again.trim().eq("y") { true } else { false };

        if playing {
            round_ct += 1;
        }
    }

    println!("\n-End Game-");
    println!("Player = {player_win}");
    println!("Comp = {comp_win}");
    println!("\n{}", if player_win > comp_win { "***Player wins the game!***" } else { "***Comp wins the game!***" });
}
