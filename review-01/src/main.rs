extern crate rand;

use std::io;
use rand::Rng;

fn main(){
    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1, 101);

    io::stdin()
        .read_line(&mut guess)
        .expect("Error Occurred!");

    let guess: u32 = guess.trim()
        .parse()
        .expect("Hello World");

    match guess.cmp(&secret_number){
        std::cmp::Ordering::Less => println!("Less"),
        std::cmp::Ordering::Equal => println!("Equal"),
        std::cmp::Ordering::Greater => println!("Greater"),
    }
}
