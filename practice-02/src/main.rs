use std::io; // standard input/output library

fn main() { // fn syntax declares a new function
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}
