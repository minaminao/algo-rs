use super::extended_gcd::*;

type T = i128;

pub fn chinese_remainder_theorem(v_a: &Vec<T>, v_n: &Vec<T>) -> (T, T) {
    assert_eq!(v_a.len(), v_n.len());
    let mut a = 0;
    let mut n = 1;
    for i in 0..v_a.len() {
        let (g, u, _) = extgcd(n, v_n[i]);
        if v_a[i] % g != a % g {
            return (-1, -1);
        }
        let m = v_n[i] / g;
        let mut t = (v_a[i] - a) / g * u % m;
        if t < 0 {
            t += m;
        }
        a += n * t;
        n *= m;
    }
    (a % n, n)
}

#[test]
fn test() {
    assert_eq!(
        chinese_remainder_theorem(&vec![1, 4, 6], &vec![3, 5, 7]),
        (34, 105)
    );
}
