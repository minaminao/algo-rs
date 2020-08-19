/// O(n log log n)
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if !is_prime[i] {
            continue;
        }
        (i * i..=n).step_by(i).for_each(|j| is_prime[j] = false);
        i += 1;
    }
    is_prime
}

#[test]
fn test() {
    let v = sieve_of_eratosthenes(10);
    assert_eq!(v[1], false);
    assert_eq!(v[2], true);
    assert_eq!(v[3], true);
    assert_eq!(v[4], false);
    assert_eq!(v[5], true);
    assert_eq!(v[6], false);
    assert_eq!(v[7], true);
    assert_eq!(v[8], false);
    assert_eq!(v[9], false);
}
