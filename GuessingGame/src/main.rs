use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand = rand::thread_rng().gen_range(1..101); //rand between 1-100

    println!("---Guess the number!---");
    //gameplay loop
    loop {
        let mut guess: String = String::new();

        //read input
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please enter a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        //compare the rand to guess
        match guess.cmp(&rand) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("~You win~");
                break;
            }
        }
        // println!("The secret number is {}", rand);
    }
}
