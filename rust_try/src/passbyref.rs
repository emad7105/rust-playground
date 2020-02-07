struct Person{
    name: String,
    age: u16
}


pub fn run(){
    let emad = Person {name: "emad".to_string(), age: 32};

    print_person_byref(&emad);
    print_person_byref(&emad);

    print_person_noref(emad);
//    print_person_noref(emad);
    // Error: not pass by ref
}


fn print_person_byref(p: &Person) {
    println! ("Name ({}) and age ({})", p.name, p.age);
}


fn print_person_noref(p: Person) {
    println! ("Name ({}) and age ({})", p.name, p.age);
}