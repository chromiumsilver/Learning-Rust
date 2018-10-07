fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}", s1, len);
    
    change(&mut s1);
    println!("{}", s1);
}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
