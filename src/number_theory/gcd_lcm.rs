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

#[test]
fn test() {
    assert_eq!(gcd(2, 4), 2);
    assert_eq!(lcm(2, 4), 4);
}
