use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    const LUCKY: i32 = 7;
    const BROKE: Decimal = dec!(0.00);

    //input strings, get shadowed later
    let mut pot: String = String::new();
    let mut bet: String = String::new();

    let mut roll_ct: i32 = 1;
    let mut lucky_ct: i32 = 0;
    let mut peak_roll: i32 = 0;
    let mut peak_pot: Decimal = dec!(0.00);
    let mut fail_exit: bool = false;

    //get buy in amount
    print!("Enter buy in: $");
    io::stdout().flush(); //only way to get the stdout and stdin on same line
    io::stdin()
        .read_line(&mut pot)
        .expect("Failed to read line");
    let mut pot: Decimal = match pot.trim().parse() {
        Ok(num) => num,
        Err(_) => BROKE,
    };

    //get bet
    print!("Enter bet [wins are tripled]: $"); // todo try this with decimals
    io::stdout().flush(); //only way to get the stdout and stdin on same line
    io::stdin()
        .read_line(&mut bet)
        .expect("Failed to read line");
    let bet: Decimal = match bet.trim().parse() {
        Ok(num) => num,
        Err(_) => BROKE,
    };

    //exit game if can't play with amounts provided
    if pot <= BROKE || bet <= BROKE {
        fail_exit = true;
        println!("Unplayable amount")
    }

    //play sim
    let win: Decimal = &bet * dec!(3.00);
    let loss: Decimal = bet;
    println!("---Rolling---");
    while pot > BROKE {
        let round: i32 = roll();
        match LUCKY.cmp(&round) {
            Ordering::Equal => {
                println!("~~~Win~~~");
                lucky_ct += 1;
                pot += win
            }
            Ordering::Greater => pot -= loss,
            Ordering::Less => pot -= loss,
        }

        println!("Roll {roll_ct} | {round} | ${pot:.2}");

        if pot > peak_pot {
            peak_pot = pot;
            peak_roll = roll_ct;
        }

        roll_ct += 1;
    }

    if !fail_exit {
        println!("You went broke after {roll_ct} rolls...");
        println!("You got lucky {lucky_ct} times tonight...");
        println!("Highest earning: Round {peak_roll} -> ${peak_pot:.2}");

        if pot < BROKE {
            let owed: Decimal = pot.abs();
            println!("You owe the house {owed:.2}");
        }
    }
}

/// Rolls 2 6-sided dice
///
/// Returns: a random i32 between 1-12
fn roll() -> i32 {
    rand::thread_rng().gen_range(1..13)
}
