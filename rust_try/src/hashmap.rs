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


    // merge 2 hashmaps
    let mut h1: HashMap<i32,i32> = HashMap::new();
    let mut h2: HashMap<i32,i32> = HashMap::new();

    h1.insert(1, 3);
    h1.insert(2, 1);
    h1.insert(4, 5);

    h2.insert(1, 2);
    h2.insert(2, 3);
    h2.insert(6, 1);
    h2.insert(7, 4);

    let mut h_merged: HashMap<i32,i32> = HashMap::new();

    for (i, j) in h1.iter() {
        h_merged.insert(i.clone(),j.clone());
    }
    for (i, j) in h2.iter() {
        if h_merged.contains_key(i) {
            let count = h_merged.get_mut(i).unwrap();
            *count += *j;
        } else {
            h_merged.insert(i.clone(), j.clone());
        }
    }

    println!("1 -> {}", h_merged.get(&1).unwrap());
    println!("2 -> {}", h_merged.get(&2).unwrap());
    println!("4 -> {}", h_merged.get(&4).unwrap());
    println!("6 -> {}", h_merged.get(&6).unwrap());
    println!("7 -> {}", h_merged.get(&7).unwrap());
    println!("len(): {}", h_merged.len());

    /*assert_eq!(*h_merged.get(&1).unwrap() as usize, 5 as usize);
    assert_eq!(*h_merged.get(&2).unwrap() as usize, 4 as usize);
    assert_eq!(*h_merged.get(&4).unwrap() as usize, 5 as usize);
    assert_eq!(*h_merged.get(&6).unwrap() as usize, 1 as usize);
    assert_eq!(*h_merged.get(&7).unwrap() as usize, 4 as usize);
    assert_eq!(h_merged.keys().len(), 5);*/
}