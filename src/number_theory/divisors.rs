/// Divisors (Unsorted)
/// O(√n), O(2^(b/2))
pub fn calc_divisors(x: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    let mut i = 1;
    while i * i < x {
        if x % i != 0 {
            i += 1;
            continue;
        }
        ret.push(i);
        ret.push(x / i);
        i += 1;
    }
    if i * i == x {
        ret.push(i);
    }
    ret
}

#[test]
fn test() {
    let mut d = calc_divisors(10);
    d.sort();
    assert_eq!(d, vec![1, 2, 5, 10]);
}
