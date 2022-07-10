extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    loop{
        let rand_number: i32 = rand::thread_rng()
                                        .gen_range(1, 101);

        println!("Rand Number : {}", &rand_number);

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Error Occurred!");

        let guess: i32 = guess.trim()
                        .parse()
                        .expect("Error Occurred!");

        match rand_number.cmp(&guess){
            std::cmp::Ordering::Less => {
                println!("Less!");
            }
            std::cmp::Ordering::Equal => { 
                println!("You win!");
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Greater!");
            }
        }
    }
}