pub fn run() {
    let mut x = 15;

    {
        let x = 10;
    }

    print!("Value x is {}", x);
}