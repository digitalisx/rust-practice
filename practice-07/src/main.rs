fn main() {
    println!("Hello, world!");
    let x = another_function(1);
    println!("{}", x);
}

fn another_function(x: i32) -> i32 {
    println!("Another function!");
    return x + 1;
}