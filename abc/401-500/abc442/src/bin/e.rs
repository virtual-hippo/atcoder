use itertools::*;
use proconio::{fastout, input, marker::*};

///
/// 偏角ソート用構造体
///
/// https://atcoder.jp/contests/abc442/tasks/abc442_e
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct V(i64, i64);

impl V {
    fn cross(&self, other: &Self) -> i64 {
        self.0 * other.1 - self.1 * other.0
    }
    fn is_up(&self) -> u8 {
        let (x, y) = (self.0, self.1);
        if y > 0 {
            0
        } else if y == 0 && x > 0 {
            0
        } else {
            1
        }
    }
}

impl PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for V {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.is_up().cmp(&other.is_up()) {
            std::cmp::Ordering::Equal => 0.cmp(&self.cross(other)),
            other => other,
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64,i64); n],
    }

    let xy = xy.iter().map(|&(x, y)| V(x, y)).collect_vec();

    // 偏角ソート
    let (idx, xy_sorted) = xy.iter().copied().enumerate().sorted_by_key(|&(_, v)| v).enumerate().fold(
        (vec![0; n], vec![]),
        |(mut idx, mut xy_sorted), (i, (j, v))| {
            idx[j] = i;
            xy_sorted.push(v);
            (idx, xy_sorted)
        },
    );

    let l = (0..n - 1).fold(vec![0; n], |mut l, i| {
        let (a, b) = (&xy_sorted[i], &xy_sorted[i + 1]);
        l[i + 1] = if a < b { i + 1 } else { l[i] };
        l
    });
    let r = (0..n - 1).rev().fold(vec![n - 1; n], |mut r, i| {
        let (a, b) = (&xy_sorted[i], &xy_sorted[i + 1]);
        r[i] = if a < b { i } else { r[i + 1] };
        r
    });

    for _ in 0..q {
        input! {
            a: Usize1,
            b: Usize1,
        }

        let (b, a) = (a, b);

        let a = l[idx[a]];
        let b = r[idx[b]];
        let b = if a < b { b } else { b + n };

        let ans = b - a + 1;
        println!("{}", ans);
    }
}
