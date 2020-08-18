use crate::data_structures::range_sum_query::*;

type I = i64;

/// Binary Indexed Tree (1-based)
/// Verified: https://atcoder.jp/contests/chokudai_s001/submissions/7949468
pub struct BinaryIndexedTree {
    n: usize,
    v: Vec<I>,
}

impl BinaryIndexedTree {
    pub fn new(n: usize) -> BinaryIndexedTree {
        let mut p = 1;
        while p < n {
            p <<= 1;
        }
        BinaryIndexedTree {
            n,
            v: vec![0; n + 1],
        }
    }

    pub fn add(&mut self, i: usize, x: I) {
        let mut i = i as isize;
        while i <= self.n as isize {
            self.v[i as usize] += x;
            i += i & -i;
            dbg!(i, -i);
        }
    }
    /// [1, r]
    pub fn sum_head(&self, i: usize) -> I {
        let mut ret = 0;
        let mut i = i as isize;
        while i > 0 {
            ret += self.v[i as usize];
            i -= i & -i;
        }
        ret
    }
}

impl RangeSumQuery for BinaryIndexedTree {
    /// [l, r]
    fn sum(&self, l: usize, r: usize) -> I {
        self.sum_head(r) - self.sum_head(l - 1)
    }
}

#[test]
fn test() {
    let mut bit = BinaryIndexedTree::new(10);
    bit.add(1, 1);
    assert_eq!(bit.sum_head(1), 1);
    bit.add(2, 2);
    assert_eq!(bit.sum_head(2), 3);
    bit.add(1, 3);
    assert_eq!(bit.sum_head(1), 4);
    assert_eq!(bit.sum_head(2), 6);
    assert_eq!(bit.sum(1, 1), 4);
    assert_eq!(bit.sum(1, 2), 6);
    assert_eq!(bit.sum(2, 2), 2);
}
