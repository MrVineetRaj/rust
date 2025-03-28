fn main() {
    let x = 5;
    println!("address of x is {:p}",&x); // info : shows the address of x
    let y = &x; // y is reference to the value of x, value of x(which is 5) is not moved
    println!("address of y is {:p}",y); // info : shows the address of x as y is reference to x

    //note : for printing address of something, we need to use {:p} in format string and pass &whatever we want to print the address of that variable 

    // note : iff we not pass {:p} in format string, it will print the value of the variable instead of address means will automatically dereference the variable 

    //info : here for y we are passing y which is a reference to x, so we will get address of x with using y and {:p} in format string

    println!("address of y is {:p}",&y); // info : shows the address of y 
}
