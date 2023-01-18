extern crate num_bigint;
extern crate num_traits;
use std::ops::Mul;
use num_bigint::{BigInt, BigUint, ToBigInt, RandBigInt};
use num_traits::{ToPrimitive, Zero};
use anyhow::Result;
use rand::rngs::ThreadRng;



pub fn run() {
    let mut sss = SecretSharing::new();
    let secret = &BigUint::from(0u128);
    let (s1, s2, s3) = sss.split(secret).expect("splitting failed.");

    let reconstructed_secret = sss.reconstruct(s1, s2).expect("reconstructing failed.");

    println!("reconstructed_secret: {:?}", reconstructed_secret);
    assert_eq!(0u128, reconstructed_secret.to_u128().unwrap());

    println!("Adding 200 + 5 ...");
    test_adding_secret();

    println!("Multiplying 200 * 5 ...");
    test_mult_secret();
}

pub fn test_adding_secret () {
    let mut sss = SecretSharing::new();
    let mut secret_5 = &BigUint::from(5u128);
    let mut secret_200 = &BigUint::from(200u128);
    let mut shares_5 = sss.split(secret_5).expect("splitting failed.");
    let mut shares_200 = sss.split(secret_200).expect("splitting failed.");

    shares_200.0.add(&shares_5.0, &sss.prime);
    shares_200.1.add(&shares_5.1, &sss.prime);
    shares_200.2.add(&shares_5.2, &sss.prime);

    let reconstructed_secret = sss.reconstruct(shares_200.0, shares_200.1).expect("reconstructing failed.");

    println!("reconstructed_secret: {:?}", reconstructed_secret);
    assert_eq!(205u128, reconstructed_secret.to_u128().unwrap())
}

pub fn test_mult_secret () {
    let mut sss = SecretSharing::new();
    let mut secret_5 = &BigUint::from(5u128);
    let mut secret_200 = &BigUint::from(200u128);
    let mut shares_5 = sss.split(secret_5).expect("splitting failed.");
    let mut shares_200 = sss.split(secret_200).expect("splitting failed.");

    shares_200.0.mul(&shares_5.0, &sss.prime);
    shares_200.1.mul(&shares_5.1, &sss.prime);
    shares_200.2.mul(&shares_5.2, &sss.prime);

    let reconstructed_secret = sss.reconstruct_2t(shares_200.0, shares_200.1, shares_200.2).expect("reconstructing failed.");

    println!("reconstructed_secret: {:?}", reconstructed_secret);
    assert_eq!(1000u128, reconstructed_secret.to_u128().unwrap())
}

trait Field {
    fn add(&mut self, rhs: &Share, prime: &BigUint);
    fn sub(&mut self, rhs: &Share, prime: &BigUint);
    fn mul(&mut self, rhs: &Share, prime: &BigUint);
    fn mul_by_2(&mut self, prime: &BigUint);
    fn mul_by_3(&mut self, prime: &BigUint);
}

#[derive(Clone)]
pub struct Share {
    value: BigUint,
    id: u32,
}

impl Field for Share {
    fn add(&mut self, rhs: &Share, prime: &BigUint) {
        self.value= SecretSharing::add(&self.value, &rhs.value, prime);
    }

    fn sub(&mut self, rhs: &Share, prime: &BigUint) {
        todo!()
    }

    fn mul(&mut self, rhs: &Share, prime: &BigUint) {
        let temp = &self.value;
        self.value = temp.mul(&rhs.value) % prime;
    }

    fn mul_by_2(&mut self, prime: &BigUint) {
        self.value = SecretSharing::mult_by_2(&self.value, prime);
    }

    fn mul_by_3(&mut self, prime: &BigUint) {
        self.value = SecretSharing::mult_by_3(&self.value, prime);
    }
}

pub struct SecretSharing {
    prime: BigUint,
    rng: ThreadRng,
}

impl SecretSharing {
    pub fn new() -> Self {
        // prime generation
        let p = BigUint::from(340282366920938463463374607431768210659u128);
        let mut rng = rand::thread_rng();

        SecretSharing {
            prime: p,
            rng,
        }
    }

    pub fn split(&mut self, secret: &BigUint) -> Result<(Share, Share, Share)> {
        // polynomial y(X) = kX + c
        let k = &self.rng.gen_biguint_below(&self.prime);

        // let share_1 = (secret + k) % &self.prime;
        let share_1 = SecretSharing::add(secret, &k, &self.prime);
        let share_2 = SecretSharing::add(secret, &SecretSharing::mult_by_2(k, &self.prime), &self.prime);
        let share_3 = SecretSharing::add(secret, &SecretSharing::mult_by_3(k, &self.prime), &self.prime);
        Ok((
            Share { value: share_1, id: 1u32 },
            Share { value: share_2, id: 2u32 },
            Share { value: share_3, id: 3u32 }
        ))
    }

    pub fn reconstruct(&self, s1: Share, s2: Share) -> Result<BigUint> {
        // s = (s2.id)*s1 - (s1.id)*s2
        // todo (Emad) use our own mult for all possible shares
        // p1 = (s2.id)*s1
        let p1 = (BigUint::from(s2.id) * s1.value) % &self.prime;
        // p2 = (s1.id)*s2
        let p2 = (BigUint::from(s1.id) * s2.value) % &self.prime;

        println!("p1: {:?}", p1);
        println!("p2: {:?}", p2);
        if p2 <= p1 {
            Ok(p1 - p2)
        } else {
            Ok(p1 - p2 + &self.prime)
        }
    }

    pub fn reconstruct_2t(&self, s1: Share, s2: Share, s3: Share) -> Result<BigUint> {
        // s = 3*s1 - 3*s2 + s3
        // s = (s3.id)*s1 - (s3.id)*s2 + s3

        // (s3.id)*s1
        // let p1 = BigUint::from(3u64) * s1.value;
        let p1 = SecretSharing::mult_by_3(&s1.value, &self.prime);

        // (s3.id)*s2
        // let p2 = BigUint::from(3u64) * s2.value;
        let p2 = SecretSharing::mult_by_3(&s2.value, &self.prime);

        let mut sum = BigUint::zero();
        if p2 <= p1 {
            sum = p1 - p2;
        } else {
            sum = p1 + &self.prime - p2;
        }

        Ok(SecretSharing::add(&sum, &s3.value, &self.prime))
    }


    fn add(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
        let mut c = a + b;
        if c >= *p {
            c -= p;
        }
        c
    }

    fn mult_by_2(a: &BigUint, p: &BigUint) -> BigUint {
        let mut c = a + a;
        if c >= *p {
            c -= p;
        }
        c
    }

    fn mult_by_3(a: &BigUint, p: &BigUint) -> BigUint {
        let mut c = a + a + a;
        if c >= *p {
            c -= p;
        }
        if c >= *p {
            c -= p;
        }
        c
    }
}