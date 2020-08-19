pub fn fibonacci(n: usize) -> Vec<usize> {
    let mut v = vec![1; n];
    (2..n).for_each(|i| v[i] = v[i - 1] + v[i - 2]);
    v
}

#[test]
fn test() {
    assert_eq!(fibonacci(7), vec![1, 1, 2, 3, 5, 8, 13]);
}
