
fn add(x: u8, y: u8) -> u8 {
    return x + y;
}
fn main() {
    println!("Hello, world!");
    let x: u8 = 255;
    // x = 10; //info : This line will cause a compilation error because `x` is immutable
    let mut y: u8 = 10;
    let mut sum: u8 = add(x, y);
    println!("The sum of {} and {} is {}", x, y, sum);
    y = 52;
    sum = add(x, y);
    println!("The sum of {} and {} is {}", x, y, sum);
}
