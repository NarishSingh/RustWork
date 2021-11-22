use core::num::dec2flt::parse;
use core::panicking::panic;
use std::io;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use rand::Rng;

fn main() {
    let mut playing: bool = true;
    let mut round_ct: i32 = 1;
    let mut player_play: String = String::new();
    let mut player_move: Moves;
    let mut comp_move: Moves;

    println!("---RPS---");
    while playing {
        println!("-Round {}-", round_ct);

        //moves
        println!("Enter move:");
        println!("1/R - Rock");
        println!("2/P - Paper");
        println!("3/S - Scissors");
        io::stdin()
            .read_line(&mut player_play)
            .expect("Failed to read line");
        player_move = match player_play.trim().as_str() {
            "1" | "R" => Moves::Rock,
            "2" | "P" => Moves::Paper,
            "3" | "S" => Moves::Scissors,
            _ => {
                println!("Invalid input");
                println!("-Terminating Round-");
                continue;
            }
        };
        println!("You chose {}", player_move);

        comp_move = get_comp_move();

        //round outcome

        //New round
        let mut play_again: String = String::new();
        print!("New round? (y/n): ");
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");
        playing = if play_again == "y" { true } else { false };
        if playing {
            round_ct += 1;
        }
    }
}

/// Get comp move
///
/// Return a random variant of the Moves enum
fn get_comp_move() -> Moves {
    let rand: i32 = rand::thread_rng().gen_range(1..4);
    match rand {
        1 => Moves::Rock,
        2 => Moves::Paper,
        3 => Moves::Scissors,
        _ => { panic("RNG Error") }
    }
}

#[derive(Debug)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

//Needed to print enum
impl Display for Moves {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
