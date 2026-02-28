use std::io;

fn main() {
    println!("Welcome to Number Guess!");

    print!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("user input should not be empty");
    
    println!("You guessed: {guess}");
}
