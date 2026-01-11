use ac_library::*;

///
/// 区間加算と区間の総和の取得を高速に行う
///
pub struct RangeFenwickTree {
    bit0: FenwickTree<i64>,
    bit1: FenwickTree<i64>,
}

impl RangeFenwickTree {
    pub fn new(n: usize) -> Self {
        // 1-indexed
        let bit0 = FenwickTree::<i64>::new(n + 1, 0);

        // 1-indexed
        let bit1 = FenwickTree::<i64>::new(n + 1, 0);

        Self { bit0, bit1 }
    }

    /// 区間 [l, r) の全要素に x を加算（0-indexed, 半開区間）
    pub fn range_add(&mut self, l: usize, r: usize, x: i64) {
        let l = l + 1;
        let r = r + 1;

        // B0 への更新
        self.bit0.add(l, x);
        self.bit0.add(r, -x);

        // B1 への更新
        self.bit1.add(l, -x * (l as i64 - 1));
        self.bit1.add(r, x * (r - 1) as i64);
    }

    /// a[1] + a[2] + ... + a[r] を取得（prefix sum, 1-indexed）
    fn prefix_sum(&self, r: usize) -> i64 {
        self.bit0.sum(1..r) * (r - 1) as i64 + self.bit1.sum(1..r)
    }

    /// 区間 [l, r) の和を取得（0-indexed, 半開区間）
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        let l = l + 1;
        let r = r + 1;
        self.prefix_sum(r) - self.prefix_sum(l)
    }

    /// 位置 i の値を取得（0-indexed）
    pub fn get(&self, i: usize) -> i64 {
        self.range_sum(i, i + 1)
    }
}
