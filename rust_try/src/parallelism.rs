use std::time::Instant;
use num_bigint::{BigUint, RandBigInt};
use rayon::prelude::*;

pub fn run() {
    let input  = get_random(1000000);

    let start_seq = Instant::now();
    sum_of_squares_sequential(&input);
    let duration_seq = start_seq.elapsed();
    println!("duration_seq: {:?}", duration_seq);

    let start_par = Instant::now();
    sum_of_squares_parallel(&input);
    let duration_par = start_par.elapsed();
    println!("duration_par: {:?}", duration_par);
}

fn sum_of_squares_parallel(input: &[BigUint]) -> BigUint {
    input.par_iter() // <-- just change that!
        .map(|i| i * i)
        .sum()
}

fn sum_of_squares_sequential(input: &[BigUint]) -> BigUint {
    input.iter() // <-- just change that!
        .map(|i| i * i)
        .sum()
}

fn get_random (num: usize) -> Vec<BigUint> {
    let mut rng = rand::thread_rng();
    let mut values = vec![];
    for i in 1..num {
        values.push(rng.gen_biguint(128u64));
    }
    values

}