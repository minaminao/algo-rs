/// Greatest common divisor
pub fn gcd(x: u64, y: u64) -> u64 {
    if y > 0 {
        gcd(y, x % y)
    } else {
        x
    }
}

/// Lowest common multiple
pub fn lcm(x: u64, y: u64) -> u64 {
    x / gcd(x, y) * y
}

pub fn gcd_array(v: &[u64]) -> u64 {
    let mut ret = v[0];
    (1..v.len()).for_each(|i| ret = gcd(ret, v[i]));
    ret
}

pub fn lcm_array(v: &[u64]) -> u64 {
    let mut ret = v[0];
    (1..v.len()).for_each(|i| ret = lcm(ret, v[i]));
    ret
}

pub fn gcd_non_recursive(mut x: u64, mut y: u64) -> u64 {
    while y > 0 {
        let t = x;
        x = y;
        y = t % y;
    }
    x
}

#[test]
fn test() {
    assert_eq!(gcd(2, 4), 2);
    assert_eq!(lcm(2, 4), 4);
}
