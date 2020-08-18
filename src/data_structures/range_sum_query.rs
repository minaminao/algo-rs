type I = i64;

pub trait RangeSumQuery {
    fn sum(&self, l: usize, r: usize) -> I;
}
