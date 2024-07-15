use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        w: [usize; n],
    }
    let mut state = HashSet::new();
    let mut sum_w = vec![];
    for bit in 0..(1 << n) {
        let mut current = vec![];
        let mut sum = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                current.push(i);
                sum += w[i];
            }
        }
        state.insert(current);
        sum_w.push(sum);
    }
    let mut map = HashMap::new();
    for vec in state.iter() {
        map.insert(vec.clone(), f64::MAX);
    }

    let mut dp = vec![map; d + 1];

    if let Some(val) = dp[0].get_mut(&vec![]) {
        *val = {
            let sum = w.iter().sum::<usize>() as f64;
            let v = sum - sum / (d as f64);
            (v * v) / (d as f64)
        };
    }

    for i in 1..d + 1 {
        for st in state.iter() {
            let prev = *(dp[i - 1].get(st).unwrap());
            if let Some(val) = dp[i].get_mut(st) {
                if *val != f64::MAX {
                    *val = val.min(prev);
                }
            }
        }
    }

    let ans = 0;
    println!("{}", sum_w.len());
}
