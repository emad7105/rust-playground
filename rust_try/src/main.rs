//mod print;
//mod vars;
//mod types;
//mod strings;
//mod tuples;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod functions;
//mod pointers;
//mod structs;
//mod enums;
//mod cli;
//mod shadowing;
//mod references;
//mod passbyref;
mod struct_serialization;
//mod hashmap;

fn main() {
    println!("Hello from main.rs");

//    print::run();
//    vars::run();
//    types::run();
//    strings::run();
//    tuples::run();
//    arrays::run();
//    vectors::run();
//    conditionals::run();
//    loops::run();
//    functions::run();
//    pointers::run();
//    structs::run();
//    enums::run();
//    cli::run();
//    shadowing::run();
//    references::run();
//    passbyref::run();
    struct_serialization::serialize_deserialize();
//    hashmap::play_hashmap();
}