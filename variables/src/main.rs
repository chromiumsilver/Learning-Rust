// variables default immutable
/*fn main() {
    let mut x = 5;
    println!{"The value of x is: {}", x};
    x = 6;
    println!{"The value of x is: {}", x};
}*/

// shadowing
/*fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
}*/

//Numeric Operations
/*fn main() {
    //addition
    let sum = 5 + 10;
    //subtraction
    let difference = 95.5 - 4.3;
    //multiplication
    let product = 4 * 30;
    //division
    let quotient = 56.7 / 32.2;
    //remainder
    let remainder = 43 % 5;
}*/

//tuple destructuring
/*fn main() {
    let tup = (500, 6.4, 1); //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //destructure a tuple
    println!("The value of y is: {}", y);
}*/

//directly access tuple elements
/*fn main() {
    let x = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);
}*/

//Array
fn main() {
    let a: [i32, 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second =a[1];
}

