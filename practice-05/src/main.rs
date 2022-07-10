fn main(){
    const DISTANCE: u32 = 100_900;

    let guess: u32 = 12;
    // Error case - guess = 13; 
    println!("{}", &guess);

    let mut number: u32 = 12; // why occurred overwritten warning?
    number = 13;
    println!("{}", &number);

    println!("{}", &DISTANCE);
}