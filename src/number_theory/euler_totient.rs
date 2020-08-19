/// Euler's totient function
pub fn euler_totient(mut n: usize) -> usize {
    let mut ret = n;
    let mut x = 2;
    while x * x <= n {
        if n % x > 0 {
            continue;
        }
        ret -= ret / x;
        while n % x == 0 {
            n /= x;
        }
        x += 1;
    }
    if n != 1 {
        ret -= ret / n;
    }
    ret
}

#[test]
fn test() {
    assert_eq!(euler_totient(12), 4);
    assert_eq!(euler_totient(180), 48);
}
