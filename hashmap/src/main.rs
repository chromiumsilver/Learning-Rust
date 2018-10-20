use std::collections::HashMap;
fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];
    let mut scores: HashMap<_,_> = teams.iter().zip(initial_score.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // if not reference, field_name, field_value are invalid
    // println!("{}, {}", field_name, field_value);

    // access HashMap
    let color = map.get(&field_name);
    println!("color: {:?}", color);

    // iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // update
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrite
    println!("{:?}", scores);
    // only insert if the key does not already have a value
    scores.entry(String::from("Yellow")).or_insert(50);  
    // update value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
