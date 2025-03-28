fn main() {
    // Note : Array is a fixed size collection of elements of same type 
    // Note : Array is a stack allocated collection of elements of same type 
    let mut arr1:[i32; 5] = [1, 2, 3, 4, 5]; // info : array of size 5 with values 
    write_arr1(arr1);
    write_arr2(&mut arr1);
    println!("Array {:?}", arr1); // info : prints the array 

    

}
fn write_arr2( arr:&mut [i32;5]){
    arr[0] = 20; 
    println!("Array {:?}", arr); // info : prints the array
}

fn write_arr1( mut arr:  [i32;5]){
    arr[0] = 10; 
    println!("Array {:?}", arr); // info : prints the array
}