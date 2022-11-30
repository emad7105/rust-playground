use std::collections::HashMap;
use std::string::ToString;
use serde::de::Unexpected::Option;

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

    for_bbmpc_paper();
}

fn for_bbmpc_paper() {

    let party_count = 4;
    let p0_index:u8 = 0;
    let p1_index:u8 = 1;
    let p2_index:u8 = 2;
    let p3_index:u8 = 3;

    let mut h: HashMap<u64,HashMap<u8, Vec<u8>>> = HashMap::new();

    //-- Task: for ctr = 1, add for p2, msg=[11,22,33]
    let ctr_1:u64 = 1;
    let msg_ctr1_p2 = vec![11,22,33];

    upsert_message_for_counter(&p2_index, &mut h, &ctr_1, &msg_ctr1_p2);
    println!("h = {:?}", h);
    // h = {1: {2: [11, 22, 33]}}

    //Task: for ctr = 1, add for p3, msg=[33,23,4]
    let msg_ctr1_p3 = vec![33,23,4];
    upsert_message_for_counter(&p3_index, &mut h, &ctr_1, &msg_ctr1_p3);
    println!("h = {:?}", h);
    // h = {1: {2: [11, 22, 33], 3: [33, 23, 4]}}

    //Task: for ctr = 1, add for p2, msg=[1,1,2]
    let msg_ctr1_p2_new = vec![1,1,2];
    upsert_message_for_counter(&p2_index, &mut h, &ctr_1, &msg_ctr1_p2_new);
    println!("h = {:?}", h);
    // h = {1: {2: [1, 1, 2], 3: [33, 23, 4]}}

    //Task: for ctr = 3, add for p2, msg=[23,44,55]
    let ctr_3:u64 = 3;
    let msg_ctr3_p2 = vec![23,44,55];
    upsert_message_for_counter(&p2_index, &mut h, &ctr_3, &msg_ctr3_p2);
    println!("h = {:?}", h);
    // h = {3: {2: [23, 44, 55]}, 1: {2: [1, 1, 2], 3: [33, 23, 4]}}

    //Task: get all messages (ctr=1)
    let messages = get_messages_for_counter(&mut h, &ctr_1);
    println!("messages(ctr=1) = {:?}", messages);
    // messages(ctr=1) = [[1, 1, 2], [33, 23, 4]]

    //Task: get all messages (ctr=3)
    let messages = get_messages_for_counter(&mut h, &ctr_3);
    println!("messages(ctr=3) = {:?}", messages);
    // messages(ctr=3) = [[23, 44, 55]]

    //Task: delete all messages (ctr=1)
    delete_messages_for_counter(&mut h, &ctr_1);
    println!("h = {:?}", h);
    // h = {3: {2: [23, 44, 55]}}

    //Task: AGAIN delete all messages (ctr=1) - it should ignore but not fail
    delete_messages_for_counter(&mut h, &ctr_1);
    println!("h = {:?}", h);
    // h = {3: {2: [23, 44, 55]}}
}

fn delete_messages_for_counter(h: &mut HashMap<u64, HashMap<u8, Vec<u8>>>, ctr: &u64) {
    h.remove(ctr);
}

fn get_messages_for_counter(h: &mut HashMap<u64, HashMap<u8, Vec<u8>>>, ctr: &u64) -> Vec<Vec<u8>> {
    let h_ctr_option = h.get(ctr);
    if h_ctr_option == None {
        Vec::new()
    } else {
        let h_ctr = h_ctr_option.unwrap();
        h_ctr.values().map(|v| v.to_vec()).collect()
    }
}

fn upsert_message_for_counter(p_i_index: &u8, h: &mut HashMap<u64, HashMap<u8, Vec<u8>>>, ctr: &u64, msg: &Vec<u8>) {
    let mut h_ctr_option = h.get_mut(&ctr);
    if h_ctr_option == None {
        let mut new_h_ctr: HashMap<u8, Vec<u8>> = HashMap::new();
        insert_h_messages(&p_i_index, &msg, &mut new_h_ctr);
        h.insert(ctr.clone(), new_h_ctr);
    } else {
        let h_ctr = h_ctr_option.unwrap();
        insert_h_messages(&p_i_index, &msg, h_ctr);
    }
}

fn insert_h_messages(p_i_index: &u8, msg: &Vec<u8>, h_ctr_messages: &mut HashMap<u8, Vec<u8>>) {
    h_ctr_messages.insert(p_i_index.clone(), msg.clone());
}