use std::fmt;
fn main() {
    println!("Hello, world!");
    println!("My name is {0}, {1} {0}", "James", "Bob");
    
    // Derive the `fmt::Debug` implementation for `Structure`.
    #[derive(Debug)]
    struct Structure(i32);
    
    //pretty printing `{:#?}.
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    println!("This struct `{:?}`", Structure(3));
    let name = "peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
    
    // control floating number
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    // customize the output appearance
    let comp = Complex { real: 3.3, imag: 7.2};
    println!("Display: {}", comp);
    println!("Debug: {:?}", comp);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real} + {imag}i", real = self.real, imag = self.imag)
    }
}

//Define a structure named `List` containing a `Vec`
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing and create a reference to `vec`
        let vec = &self.0;
        write!(f, "[")?;
        // Iterate over `vec` in `v` while enumerating the iteration
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RBG ({}, {}, {}) ", self.red, self.green, self.blue)?;
        write!(f, "0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue) 
    }
}
