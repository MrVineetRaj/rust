fn main() {
    let mut s1:String = String::from("hello");
    // info : below code will not give any error because we are performing write operation with synchronous manner
    {
        let w1 = &mut s1;
        w1.push_str(", world");
        println!("{}", w1);

        let w2 = &mut s1; 
        w2.push_str("!!!");
        println!("{}", w2);
    }  


    {
        let w1 = &mut s1;
        w1.push_str("!!!");
        let w2 = &mut s1; // info : currently w1 has the reference of s2 so we can't transfer the ownership of s1 to w2
        println!("{} {}", w1,w2); // error : w1 and w2 are mutable references to s1
    }
}
