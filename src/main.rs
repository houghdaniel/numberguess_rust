use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Number Guess!");

    println!("I am thinking of a number from 1 to 100...");
    let rand_num: u8 = rand::random_range(1..100);
    println!("SECRET: Random number = {rand_num}");

    let mut n_guess: u8 = 7;
    while n_guess > 0 {
        println!("You have {} guesses remaining.", n_guess);
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin");

        // Parse input string into u8 in range 1..100
        let guess_u8: u8 = guess.trim()
            .parse::<u8>()
            .expect("Input should be an integer");
        match guess_u8 {
            1..100 => {
                if guess_u8 == rand_num {
                    println!("Your guess of {} was correct!", guess_u8);
                    break;
                } else {
                    println!("Your guess of {} was incorrect.", guess_u8);
                    n_guess -= 1;
                }
            },
            _ => {
                eprintln!("Guess should be between 1 and 100 (inclusive).");
                continue
            }
        }
        
        
    }
    


}
