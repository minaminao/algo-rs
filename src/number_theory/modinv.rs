use crate::number_theory::modpow::*;

type U = u128;

pub fn modinv(a: U, m: U) -> U {
    modpow(a, m - 2, m)
}
