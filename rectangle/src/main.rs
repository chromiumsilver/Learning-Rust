fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    println!("square is {:?}", sq);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// impl: implementation
// &mut self to change the instance
impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions, often used for constructors
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}*/
