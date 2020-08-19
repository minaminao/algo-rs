/// Calc the array of the number of divisors
/// O(n log n): n + n / 2 + n / 3 + ... + n / n = n log n
pub fn calc_array_of_number_of_divisors(n: usize) -> Vec<usize> {
    let mut num = vec![0; n + 1];
    (1..=n).for_each(|i| {
        (i..=n).step_by(i).for_each(|j| {
            num[j] += 1;
        })
    });
    num
}

#[test]
fn test() {
    use crate::number_theory::divisors::*;
    let n = 10;
    let number_of_divisors = calc_array_of_number_of_divisors(n);
    (0..=n).for_each(|x| assert_eq!(number_of_divisors[x], calc_divisors(x).len()));
}
