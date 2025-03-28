fn main() {

    let mut y = 0;
    while y < 5 {
        println!("y is {}", y);
        y += 1;
    }

    // infinite loop
    // loop {
    //     let x = 5;

    //     if x > 0 {
    //         println!("x is positive");
    //     } else if x < 0 {
    //         println!("x is negative");
    //     } else {
    //         println!("x is zero");
    //     }
    // }

    // for loop
    let arr = [1,2,3];

    for item in &arr {
        println!("item is {}", item);
    }
}
