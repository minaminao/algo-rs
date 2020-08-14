#![allow(dead_code)]
/// Binary Indexed Tree (1-based)
/// Verified: https://atcoder.jp/contests/chokudai_s001/submissions/7949468
struct BinaryIndexedTree {
    n: usize,
    p: usize,
    v: Vec<isize>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> BinaryIndexedTree {
        let mut p = 1;
        while p < n {
            p <<= 1;
        }
        BinaryIndexedTree {
            n,
            p,
            v: vec![0; n + 1],
        }
    }

    fn add(&mut self, i: usize, x: isize) {
        let mut i = i as isize;
        while i <= self.n as isize {
            self.v[i as usize] += x;
            i += i & -i;
            dbg!(i, -i);
        }
    }
    /// [1, r]
    fn sum(&mut self, i: usize) -> isize {
        let mut ret = 0;
        let mut i = i as isize;
        while i > 0 {
            ret += self.v[i as usize];
            i -= i & -i;
        }
        ret
    }
    /// [l, r]
    fn sum2(&mut self, l: usize, r: usize) -> isize {
        self.sum(r) - self.sum(l - 1)
    }
}

#[test]
fn test() {
    let mut bit = BinaryIndexedTree::new(10);
    bit.add(1, 1);
    assert_eq!(bit.sum(1), 1);
    bit.add(2, 2);
    assert_eq!(bit.sum(2), 3);
    bit.add(1, 3);
    assert_eq!(bit.sum(1), 4);
    assert_eq!(bit.sum(2), 6);
    assert_eq!(bit.sum2(1, 1), 4);
    assert_eq!(bit.sum2(1, 2), 6);
    assert_eq!(bit.sum2(2, 2), 2);
}
