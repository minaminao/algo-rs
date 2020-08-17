type I = i64;

pub struct SparseTable {
    a: Vec<I>,
    mini: Vec<Vec<I>>,
    lg: Vec<usize>,
}

impl SparseTable {
    /// O(n log n)
    pub fn new(a: &Vec<I>) -> Self {
        let n = a.len();
        let lg_n = (n as f64).log2() as usize + 1;
        let mut st = Self {
            a: a.clone(),
            mini: vec![vec![0; n]; lg_n],
            lg: vec![0; n + 1],
        };
        (2..=n).for_each(|i| {
            st.lg[i] = st.lg[i >> 1] + 1;
        });
        (0..n).for_each(|i| {
            st.mini[0][i] = st.a[i];
        });
        (0..lg_n - 1).for_each(|k| {
            (0..(n - (1 << k))).for_each(|i| {
                st.mini[k + 1][i] = std::cmp::min(st.mini[k][i], st.mini[k][i + (1 << k)]);
            })
        });
        st
    }
    /// O(1) [l, r]
    pub fn query(&self, l: usize, r: usize) -> I {
        if r - l <= 0 {
            return I::MAX;
        }
        let k = self.lg[r - l];
        std::cmp::min(self.mini[k][l], self.mini[k][r - (1 << k)])
    }
}

#[test]
fn test() {
    let a = vec![7, 2, 3, 0, 5, 10, 3, 12, 18];
    let sparse_table = SparseTable::new(&a);
    assert_eq!(sparse_table.query(0, 4), 0);
    assert_eq!(sparse_table.query(4, 7), 3);
    assert_eq!(sparse_table.query(7, 8), 12);
}
