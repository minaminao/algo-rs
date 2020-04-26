fn gcd(x: u64, y: u64) -> u64 {
    if y > 0 {
        gcd(y, x % y)
    } else {
        x
    }
}

#[test]
fn test() {
    assert_eq!(gcd(2, 4), 2);
}
