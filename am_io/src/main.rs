use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter a string : ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // let input = input.trim();
    println!("You entered: {}", input);

}
