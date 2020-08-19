const EPS: f64 = 1e-8;
pub fn modfloat(mut x: f64, m: f64) -> f64 {
    x -= (x / m).floor() * m;
    if x < EPS || x + EPS > m {
        x = 0.;
    }
    x
}

#[test]
fn test() {
    let res = modfloat(10., 3.);
    assert!(1. - EPS < res && res < 1. + EPS);
}
