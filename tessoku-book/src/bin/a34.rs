use proconio::input;
use std::collections::HashSet;

fn main() {
    const N: usize = 100_005;
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut g = vec![0; N * 2];
    for i in 0..N {
        let mut state = HashSet::new();
        if i >= x {
            state.insert(g[i - x]);
        }
        if i >= y {
            state.insert(g[i - y]);
        }
        while state.contains(&g[i]) {
            g[i] += 1;
        }
    }

    let mut v = 0;
    for _ in 0..n {
        input! {
            a: usize,
        }
        v ^= g[a];
    }
    if v == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
