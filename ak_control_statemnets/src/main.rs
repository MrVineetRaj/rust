fn main() {
    let x = 2;

    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }

    fn is_even(n: i32) -> bool {
        return n % 2 == 0
    }


    // match statement
    match x {
        1 => println!("x is one"),
        2 | 4 => println!("x is two or 4"),
        3 => println!("x is three"),
        _ => println!("x is something else"),
    }// match statement
    let num  = 28;
    match num {
        x if is_even(x) => println!("x is Even"),
        _ => println!("x is odd"),
    }
}
