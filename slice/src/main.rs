fn main() {
    let mut s = String::from("hello, world");
    let word = first_word(&s);
    println!("{}", word); 
    let string_literal = "hello, world";
    let word_literal = first_word(&string_literal);
    println!("{}", word); 
    // s.clear(); // tring to mutable reference
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
