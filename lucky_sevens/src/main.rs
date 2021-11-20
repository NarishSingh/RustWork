use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    let lucky: i32 = 7;
    let win: i32 = 4;
    let loss: i32 = 1;

    let mut pot: String = String::new();
    let mut roll_ct: i32 = 1;
    let mut lucky_ct: i32 = 0;
    let mut peak_roll: i32 = 0;
    let mut peak_pot: i32 = 0;

    //get buy in amount
    print!("Enter buy in: $");
    io::stdout().flush(); //only way to get the stdout and stdin on same line
    io::stdin()
        .read_line(&mut pot)
        .expect("Failed to read line");
    let mut pot: i32 = match pot.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    //play sim
    println!("---Rolling---");
    while pot > 0 {
        let round: i32 = roll();
        println!("Roll {} | {}", roll_ct, round);

        match lucky.cmp(&round) {
            Ordering::Equal => {
                println!("~~~Win~~~");
                lucky_ct += 1;
                pot += win
            }
            Ordering::Greater => pot -= loss,
            Ordering::Less => pot -= loss,
        }

        if pot > peak_pot {
            peak_pot = pot;
            peak_roll = roll_ct;
        }

        roll_ct += 1;
    }

    println!("You went broke after {} rolls...", roll_ct);
    println!("You got lucky {} times tonight...", lucky_ct);
    println!("Highest earning: Round {} -> ${}", peak_roll, peak_pot);
}

/// Rolls 2 6-sided dice
///
/// Returns: a random i32 between 1-12
fn roll() -> i32 {
    rand::thread_rng().gen_range(1..13)
}
