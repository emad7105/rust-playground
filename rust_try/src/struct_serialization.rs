//extern crate serde_derive;
extern crate serde;
extern crate bincode;

use serde::{Serialize, Deserialize};
use self::serde::{Serializer, Deserializer};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Person {
    name: String,
    age: i32,
    gender: String,
    picture: Vec<u8>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Hospital {
    name: String,

    #[serde(skip_serializing, skip_deserializing)]
    doc: Doctor,
}


struct Doctor {
    name: String,
    age: i32,
}


pub fn serialize_deserialize() {
    let p1 = Person {
        name: "Emad".to_string(),
        age: 12,
        gender: "Male".to_string(),
        picture: vec![1, 2, 3, 4],
    };

    let serialized_p1 = bincode::serialize(&p1).unwrap();
    let deserialized_p1: Person = bincode::deserialize(&serialized_p1).unwrap();

    assert!(p1.name == deserialized_p1.name);
    assert!(p1.age == deserialized_p1.age);
    assert!(p1.gender == deserialized_p1.gender);


    let uzleuven = Hospital {
        name: "UZ Leuven".to_string(),
        doc: Doctor {
            name: "Dr. Dre".to_string(),
            age: 45,
        },
    };
}