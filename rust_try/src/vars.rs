// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Emad";
    let mut age = 29;
    println!("wrong age: {}", age);

    age = 30;

    println!("My name is {}, age: {}", name, age);

    // Constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Brad", 37);
    println!("{} is {}", my_name, my_age );
}