pub fn log_factorial(n: usize) -> f64 {
    if n < 100 {
        return (1..=n).map(|i| (i as f64).ln()).sum();
    }
    let n = n as f64;
    n * n.ln() - n + (2. * std::f64::consts::PI * n).ln() / 2. + 1. / (12. * n)
}
