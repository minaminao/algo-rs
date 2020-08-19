/// O(sqrt n) if the number of bits of n is fixed.
pub fn is_prime(x: usize) -> bool {
    if x <= 1 {
        false
    } else if x == 2 {
        true
    } else if x % 2 == 0 {
        false
    } else {
        let mut i = 3;
        while i * i <= x {
            if x % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
}
