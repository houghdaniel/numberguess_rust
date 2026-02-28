use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Number Guess!");

    println!("I am thinking of a number from 1 to 100...");
    let rand_num: u8 = rand::random_range(1..100);
    //println!("SECRET: Random number = {rand_num}");

    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("user input should not be empty");
    
    println!("You guessed: {guess}");
}
