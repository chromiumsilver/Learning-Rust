fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divided by 4");
    } else if number % 3 == 0 {
        println!("number is divided by 3");
    } else if number % 2 == 0 {
        println!("number is divided by 2");
    }
    else {
        println!("number is not divided by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
    loop_exp();
    while_exp();
    for_exp();
}

//loop
fn loop_exp() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

//while
fn while_exp() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("liftoff!");
}

//for
fn for_exp() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
