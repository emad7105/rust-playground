use std::collections::HashMap;
use std::string::ToString;

pub fn play_hashmap() {
    let mut book_reviews: HashMap<i32,String> = HashMap::new();

    book_reviews.insert(3, "Emad11".to_string());
    book_reviews.insert(10, "Emad22".to_string());
    book_reviews.insert(40, "Emad3".to_string());
    book_reviews.insert(1, "Emad44".to_string());
    book_reviews.insert(23, "Emad43".to_string());


    let key = 1;
    assert_eq!(&"Emad44".to_string(), book_reviews.get(&key).unwrap());
}