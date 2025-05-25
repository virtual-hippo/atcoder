use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        a: [usize; 2 * n],
    }
    let mut used = vec![false; 2 * n];
    let mut heap = BinaryHeap::new();
    let mut ans = 0;

    ans += a[0];
    used[0] = true;
    let mut k = 1;

    while k < n {
        let (i, j) = (2 * k - 1, 2 * k);
        heap.push((a[i], i));
        heap.push((a[j], j));

        if let Some((v, idx)) = heap.pop() {
            used[idx] = true;
            ans += v;
        }

        k += 1;
    }

    println!("{}", ans);
}
