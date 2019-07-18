use rand::Rng;
use num::BigUint;
use num::traits::cast::ToPrimitive;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    BigUint::from(g).modpow(&a.into(), &p.into()).to_u64().unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    BigUint::from(b_pub).modpow(&a.into(), &p.into()).to_u64().unwrap()
}
