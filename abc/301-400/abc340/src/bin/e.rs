use ac_library::*;
use itertools::*;
use proconio::{fastout, input};

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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [usize; m],
    }

    let mut tree = RangeFenwickTree::new(n);

    for i in 0..n {
        tree.range_add(i, i + 1, a[i]);
    }

    for i in 0..m {
        let x = tree.get(b[i]) as usize;
        tree.range_add(b[i], b[i] + 1, -(x as i64));
        tree.range_add(0, n, (x / n) as i64);

        let x = x % n;
        if x == 0 {
            continue;
        }
        let l = (b[i] + 1) % n;
        let r = (b[i] + x + 1) % n;

        if l < r {
            tree.range_add(l, r, 1);
        } else {
            tree.range_add(l, n, 1);
            tree.range_add(0, r % n, 1);
        }
    }

    let ans = (0..n).map(|i| tree.get(i)).collect_vec();
    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
