use ac_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let mut mn_tree = Segtree::<Min<i64>>::new(n);
    let mut mx_tree = Segtree::<Max<i64>>::new(n);
    for i in 0..n {
        mn_tree.set(i, h[i]);
        mx_tree.set(i, h[i]);
    }

    let mut ans = 0;
    for i in 0..(n - k + 1) {
        let mn = mn_tree.prod(i..i + k);
        let mx = mx_tree.prod(i..i + k);
        ans = ans.max(mx - mn);
    }

    println!("{}", ans);
}
