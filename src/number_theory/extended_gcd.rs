type I = i128;

pub fn extgcd(a: I, b: I) -> (I, I, I) {
    let mut g = a;
    let mut x = 1;
    let mut y = 0;
    if b != 0 {
        let (tg, tx, ty) = extgcd(b, a % b);
        g = tg;
        x = ty;
        y = tx;
        y -= a / b * x;
    }
    (g, x, y)
}

#[test]
fn test() {
    let (g, _x, _y) = extgcd(5, 10);
    assert_eq!(g, 5);
}
