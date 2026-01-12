use itertools::*;
use proconio::{fastout, input};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [i64; n],
    }

    let a = a.iter().copied().sorted_by_key(|&x| std::cmp::Reverse(x)).collect_vec();
    let mut heap = BinaryHeap::new();

    let score = |d: &[i64]| (0..n).map(|i| a[i] * d[i]).sum::<i64>();

    {
        let mut d = vec![0_i64; n];
        d[0] = k as i64;
        heap.push((score(&d), d.clone()));
    }

    let mut ans = vec![];
    while let Some((s, mut d)) = heap.pop() {
        ans.push(s);
        if ans.len() == x {
            break;
        }

        let i = (0..n).rev().find(|&i| d[i] > 0).unwrap();
        if d[n - 1] == 0 {
            d[i] -= 1;
            d[i + 1] += 1;
            heap.push((score(&d), d.clone()));
            d[i] += 1;
            d[i + 1] -= 1;
        }
        if i > 0 && d[i - 1] > 0 {
            d[i - 1] -= 1;
            d[i] += 1;
            heap.push((score(&d), d.clone()));
        }
    }

    for i in 0..x {
        println!("{}", ans[i]);
    }
}
