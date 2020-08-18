type I = i64;
// TODO: RMQ trait

pub struct SegmentTree {
    n: usize,
    d: Vec<I>,
}

impl SegmentTree {
    pub fn new(m: usize) -> Self {
        let n = {
            let mut n = 1;
            while n < m {
                n *= 2;
            }
            n
        };
        let d = vec![I::MAX; n * 2];
        Self { n, d }
    }
    pub fn update(&mut self, i: usize, x: I) {
        self.d[self.n + i] = x;
        let mut j = (self.n + i) / 2;
        while j > 0 {
            self.d[j] = std::cmp::min(self.d[j * 2], self.d[j * 2 + 1]);
            j >>= 1;
        }
    }
    /// [l, r]
    pub fn query(&self, mut l: usize, mut r: usize) -> I {
        let mut mi = self.get(l);
        let bl = (l as isize & -(l as isize)) as usize;
        let br = (r as isize & -(r as isize)) as usize;
        while l > 0 && l + bl <= r {
            mi = std::cmp::min(mi, self.d[(self.n + l) / bl]);
            l += bl;
        }
        while l < r {
            mi = std::cmp::min(mi, self.d[(self.n + r) / br - 1]);
            r -= br;
        }
        mi
    }
    pub fn get(&self, i: usize) -> I {
        self.d[self.n + i]
    }
}

#[test]
fn test() {
    let a = vec![7, 2, 3, 0, 5, 10, 3, 12, 18];
    let mut segment_tree = SegmentTree::new(a.len());
    for (i, &x) in a.iter().enumerate() {
        segment_tree.update(i, x);
    }
    assert_eq!(segment_tree.query(0, 4), 0);
    assert_eq!(segment_tree.query(4, 7), 3);
    assert_eq!(segment_tree.query(7, 8), 12);
}
