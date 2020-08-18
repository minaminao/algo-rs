type I = i64;

pub trait RangeMinimumQuery {
    /// [l, r)
    fn query(&self, l: usize, r: usize) -> I;
}
