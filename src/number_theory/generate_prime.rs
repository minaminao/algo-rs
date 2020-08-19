use super::miller_rabin_primality_test::*;
use rand::Rng;

type U = u128;

pub fn generate_prime(n: usize) -> U {
    let mut rng = rand::thread_rng();
    let mut x = rng.gen::<U>() % 1 << (n + 1);
    if x % 2 == 0 {
        x += 1;
    }
    while !miller_rabin_primality_test(x, Some(10)) {
        x += 2;
    }
    x
}

#[test]
fn test() {
    use super::is_prime::*;
    (0..10).for_each(|_| {
        let prime = generate_prime(5);
        assert_eq!(is_prime(prime as usize), true);
    });
}
