use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    // 約数列挙
    const M: usize = 100_005;
    let mut divs = vec![vec![]; M];
    for i in 1..M {
        let mut j = i * 2;
        while j < M {
            divs[j].push(i);
            j += i;
        }
    }

    let mut g = vec![0; M];
    // grundy 数を求める
    for i in 1..M {
        let mut set = HashSet::new();
        for d in divs[i].iter() {
            set.insert(g[*d]);
        }
        while set.contains(&g[i]) {
            g[i] += 1;
        }
    }

    let mut v = 0;
    for i in 0..n {
        v ^= g[a[i]];
    }

    let ans = if v == 0 { "Bruno" } else { "Anna" };
    println!("{}", ans);
}
