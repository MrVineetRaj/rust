fn main() {
    let mut v: Vec<i32> = Vec::<i32>::new();
    let  _v1: Vec<i32> = vec![1, 2, 3, 4, 5];

    v.push(6);
    v.push(7);  
    v.push(8);
    let item:Option<i32> = v.pop();

    println!("The item popped is: {:?}", item);

    println!("The vector is: {:?}", v);
}
