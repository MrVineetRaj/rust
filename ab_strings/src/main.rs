fn main() {
    // let string_literal = "Hello, world!"; //info : This is a string literal with type `&str` and it is immutable

    let mut new_string = String::from("Hello Un"); //info : This is a `String_data_type` type, which is mutable
    println!("my string literal is : {} ",new_string);

    new_string=String::from("Hello, Rust!"); 
    println!("my string literal is : {} ",new_string);
}
