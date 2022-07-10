
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = tup;
    
    println!("{}", x);
    println!("{}", tup.1);
}
