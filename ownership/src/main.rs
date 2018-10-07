fn main() {
    let mut s1 = String::from("hello");
    let mut s2 = s1; // s1 is no longer valid, "move"
    s2.push_str(", world!"); // appends a literal to a String
    println!("{}", s2);
    
    takes_ownership(s2);
   // println!("{}", s2); // s2 has been dropped

    let s3 = gives_ownership();
    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    
    let s6 = String::from("hello");
    let (s7, len) = calculate_length(s6);
    println!("The length of '{}' is {}.", s7, len);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
    
    // deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
