fn main() {
    let emp_info:(&str,u8) = ("John Doe", 30); //info : This is a tuple with two elements: a string for name and an integer for age

    println!("Employee Name: {}", emp_info.0); //info : Accessing the first element of the tuple

    //info : destructuring the tuple
    let (name, age) = emp_info; //info : This line destructures the tuple into two variables: `name` and `age`
    println!("Employee Name: {} and his ages is :{}", name,age); //info : Accessing the first element of the tuple
}
