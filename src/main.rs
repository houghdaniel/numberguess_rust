use std::io;
use rand::Rng;

enum State {
    RUNNING,
    WON,
    LOST,
}

fn main() {
    println!("Welcome to Number Guess!");

    println!("I am thinking of a number from 1 to 100...");
    let rand_num: u8 = rand::random_range(1..=100);
    let low:  u8 = rand_num - 1;
    let high: u8 = rand_num + 1;
    println!("SECRET: Random number = {rand_num}");

    let mut n_guess: u8 = 7;
    let mut state: State = State::RUNNING;
    loop {
        match state {
            State::RUNNING => {
                println!("You have {} guesses remaining.", n_guess);
                println!("Please input your guess: ");

                let mut guess = String::new();
                io::stdin()
                   .read_line(&mut guess)
                   .expect("Failed to read from stdin");

                // Parse input string into u8
                let guess_u8: u8 = guess.trim()
                    .parse::<u8>()
                    .expect("Input should be an integer");
                

                if guess_u8 == rand_num {
                    // Guess is correct
                    println!("Your guess of {} was correct!", guess_u8);
                    n_guess -= 1;
                    state = State::WON;

                } else if guess_u8 >= 1 && guess_u8 < rand_num {
                    // Guess is too low
                    println!("Your guess of {} was too low.", guess_u8);
                    n_guess -= 1;

                } else if guess_u8 > rand_num && guess_u8 <= 100 {
                    // Guess is too high
                    println!("Your guess of {} was too high.", guess_u8);
                    n_guess -= 1;

                } else {
                    // Guess is not within range
                    eprintln!("Guess should be between 1 and 100 (inclusive).");
                }

                // Set game state to State::LOST if player has used all guesses
                if n_guess < 1 {
                    state = State::LOST;
                }
            },

            State::WON => {
                let guesses_used: u8 = 7 - n_guess;
                if guesses_used == 1 {
                    println!("You won in 1 guess!");
                } else {
                    println!("You won in {} guesses!", guesses_used);
                }
                
                break;
            },

            State::LOST => {
                println!("You ran out of guesses and lost.");
                break;
            },
        }
    }
    
}
