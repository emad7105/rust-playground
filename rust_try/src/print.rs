pub fn run() {
    // wrong println!(1);
    println!("My number: {}", 1000);
    println!("Hello! my name is {0} and my surname is {1 }", "Walter", "White");
    println!("Hello! my name is {name} and my surname is {surname}",
             name="Walter",
             surname="White");

    println!("Binary: {:b}, Hex: {:x}, Octal:{:o}", 10, 10, 10);


    println!("{:?}", ("Emad", true, 100)); 
}