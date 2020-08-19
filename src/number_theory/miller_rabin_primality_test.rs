use super::modpow::*;
use rand::Rng;

type U = u128;

/// Miller-Rabin primality test.
/// modmul is late, so it should not be used in this function.
pub fn miller_rabin_primality_test(n: U, iteration: Option<usize>) -> bool {
    let iteration = iteration.unwrap_or(5);
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut rng = rand::thread_rng();

    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for _ in 0..iteration {
        let a = rng.gen::<U>() % (n - 1) + 1;
        let mut t = d;
        let mut m = modpow(a, t, n);
        while t != n - 1 && m != 1 && m != n - 1 {
            m = m * m % n;
            t *= 2;
        }
        if m != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    assert_eq!(miller_rabin_primality_test(1, None), false);
    assert_eq!(miller_rabin_primality_test(2, None), true);
    assert_eq!(miller_rabin_primality_test(3, None), true);
    assert_eq!(miller_rabin_primality_test(4, None), false);
    assert_eq!(miller_rabin_primality_test(5, None), true);
    assert_eq!(miller_rabin_primality_test(6, None), false);
    assert_eq!(miller_rabin_primality_test(7, None), true);
    assert_eq!(miller_rabin_primality_test(8, None), false);
    assert_eq!(miller_rabin_primality_test(9, None), false);
}
