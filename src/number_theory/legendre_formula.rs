/// The number of times to divide n! by the prime factor p
pub fn largest_power_prime(n: usize, p: usize) -> usize {
    assert!(p >= 2);
    let mut ret = 0;
    let mut q = p;
    while q <= n {
        ret += n / q;
        q *= p;
    }
    ret
}

#[test]
fn test() {
    assert_eq!(largest_power_prime(6, 2), 4);
    assert_eq!(largest_power_prime(6, 3), 2);
    assert_eq!(largest_power_prime(6, 5), 1);
}
