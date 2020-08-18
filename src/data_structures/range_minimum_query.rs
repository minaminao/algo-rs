type I = i64;

pub trait RangeMinimumQuery {
    fn update(i: usize, x: I);
    /// [l, r)
    fn query(l: usize, r: usize) -> I;
}
