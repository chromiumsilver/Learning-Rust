fn main() {
    another_function(5);
    expression();
    let x = five();
    println!("The value of x is: {}", x);
    let z = plus_one(6);
    println!("The value of z is :{}", z);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn expression() {
    let _x = 5; // let is a statement
    // {} is an expression
    let y = {
        let x = 3;
        x + 1 // no semicolon here
    };
    println!("The value of y is: {}", y);
}

// the return value of a function is synonymous with the value of the final
// expression in the block of the body of the function
fn five() -> i32 {
    5
}

//statements return (), the empty tuple
fn plus_one(x: i32) -> i32 {
    x + 1;
}
