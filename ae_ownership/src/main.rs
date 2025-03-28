fn main() {
    // info : her str is stored in the heap

    let str1 = String::from("Hello");  //info : str1 is owner of the Hello
    let str2 = str1;  //error : ownership is transferred : str2 is now the owner of the Hello

    println!("str1: {}", str1); //error : str1 is no longer valid
    println!("str2: {}", str2); 
}
