pub fn run() {
    let mut x = 10;

    //let xr = &x;

    //print!("Value of (x, xr) = ({},{})", x, xr);

    // Error: xr is not mutable
    // xr += 1;

    {
        let mutxr = &mut x;

        *mutxr += 1;
    }

    println!("Value of (x) is {}", x);

    let mut y = 10;
    let muty = &mut y;
    *muty += 1;
    println!("Value of (y) is {}", y);
}