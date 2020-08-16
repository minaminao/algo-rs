type U = u128;

pub fn modpow(mut a: U, mut e: U, m: U) -> U {
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    res
}
