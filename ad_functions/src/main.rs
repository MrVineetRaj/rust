fn print_value(value: i32) {
    println!("The value is: {}", value);
}

fn add_one(value: &i32) {
    println!("Adding one to the value: {}", value + 1);
}
fn main() {
    let val = 502;
    print_value(val);
    add_one(&val);
}

