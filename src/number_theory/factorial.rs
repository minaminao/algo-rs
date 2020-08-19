pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[test]
fn test() {
    assert_eq!(factorial(10), 3628800);
}
