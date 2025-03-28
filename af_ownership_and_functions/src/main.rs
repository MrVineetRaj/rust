
fn main(){
    let x:String = String::from("Hello");
    println!("x ka length: {}", calculate_len(&x)); //info : ownership is not transferred to the function 
    //Info : we can also pass clone of the string to not transfer ownership


    let y = x.clone(); //info : y is a clone of x
    println!("y ka length: {}", calculate_len(&y)); //info : ownership is not transferred to the function

    process_string(x); //error : ownership of x is transferred to the function

    // println!("x: {}", x); //error : x is no longer valid

    let mut z = String::from("Hello Universe");
    append_string(&mut z); //info : ownership of z is not transferred to the function
    println!("z: {}", z); //info : z is still valid

}

fn process_string(s: String) {
    println!("Processing string: {}", s);
}

fn  calculate_len(s: &String) -> usize { 
    //info : ownership is not transferred to the function instead a reference is passed
    //info : s is a reference to the string
    // info : if s goes out of scope, the original string is not dropped 
    // info : also called borrowing
    //info : s is a reference so it is not valid to change the value of s
    
    // s.push_str(" World"); //error : cannot borrow `*s` as mutable, as it is behind a `&` reference 

    return s.len()
}


fn append_string(s: &mut String) {
    
    s.push_str(" with love"); //info : we can change the value of s because it is a mutable reference
}