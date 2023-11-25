use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
    }
    let mut ca = Vec::with_capacity(n);
    let mut min_cost = HashMap::new();
    min_cost.insert(vec![0; k], 0);
    for _ in 0..n {
        input! {
            c: i64,
            a: [usize; k],
        }

        // for from in vec![(0..=p).rev(); k].into_iter().multi_cartesian_product() {
        //     if let Some(from_cost) = min_cost.get(&from) {
        //         let mut to = from.clone();
        //         for i in 0..k {
        //             to[i] = p.min(to[i] + a[i]);
        //         }
        //         let cost = c + from_cost;
        //         if !min_cost.contains_key(&to) || min_cost[&to] > cost {
        //             min_cost.insert(to, cost);
        //         }
        //     }
        // }
        ca.push((c, a));
        break;
    }
    let ans = min_cost.get(&vec![p; k]).unwrap_or(&-1);
    println!("{}", ans);
    // println!("{}", 1 << 2); // 0b100
    // let mut dp = vec![vec![100_000_000_000_usize; (1 << k * 4) + 1]; n + 1];
    // dp[0][0] = 0;
    // for i in 1..n + 1 {
    //     for j in 1..k + 1 {
    //         for kk in (1 << (j - 1) * 4)..(1 << j * 4) {
    //             dp[i][kk] = dp[i][kk].min(dp[i - 1][kk]);
    //         }
    //         // dp[i][1 << j * 4] = dp[i][1 << j * 4].min(dp[i - 1][1 << j * 4]);
    //         // if 7 & dp[i - 1][1 << j * 4]
    //         // a[i - 1][j - 1]
    //     }
    // }
}
