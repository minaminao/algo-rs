use super::modpow::*;

type U = u128;

/// Legendre symbol.
fn legendre(a: U, p: U) -> i128 {
    let l = modpow(a, (p - 1) / 2, p);
    if l == p - 1 {
        -1
    } else {
        l as i128
    }
}

/// Tonelli–Shanks algorithm.
pub fn modsqrt(a: U, p: U) -> U {
    let l = legendre(a, p);
    if l != 1 {
        return l as u128;
    }

    let mut s = 0;
    let mut q = p - 1;
    while (q & 1) == 0 {
        s += 1;
        q >>= 1;
    }

    let mut z = 1;
    while legendre(z, p) == 1 {
        z += 1;
    }

    let mut m = s;
    let mut c = modpow(z, q, p);
    let mut t = modpow(a, q, p);
    let mut r = modpow(a, (q + 1) / 2, p);

    while t != 1 {
        for i in 1..m {
            if modpow(t, 1 << i, p) != 1 {
                continue;
            }
            let b = modpow(c, 1 << (m - i - 1), p);
            m = i;
            c = b * b % p;
            t = t * b % p * b % p;
            r = r * b % p;
            break;
        }
    }
    return r;
}

#[test]
fn test() {
    assert_eq!(modsqrt(5, 41), 28);
}
