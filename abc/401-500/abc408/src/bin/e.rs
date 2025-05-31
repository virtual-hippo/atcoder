use ac_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut edges = vec![];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: i64,
        }
        let a = a - 1;
        let b = b - 1;
        graph[a].push((b, c));
        graph[b].push((a, c));
        edges.push((a, b, c));
    }

    let mut x = (1 << 30) - 1;
    for k in (0..30).rev() {
        x ^= 1 << k;
        let mut dsu = Dsu::new(n);
        for (a, b, c) in edges.iter() {
            if x | c == x {
                dsu.merge(*a, *b);
            }
        }
        if !dsu.same(0, n - 1) {
            x |= 1 << k;
        }
    }

    let ans = x;
    println!("{}", ans);
}
