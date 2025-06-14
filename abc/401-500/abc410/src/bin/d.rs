use proconio::{fastout, input};
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            w: usize,
        }
        let a = a - 1;
        let b = b - 1;
        graph[a].push((b, w));
    }

    let mut dist = vec![HashSet::new(); n];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    dist[0].insert(0);

    while let Some(a) = queue.pop_front() {
        for &(b, w) in graph[a].iter() {
            let mut next = vec![];
            for &bit in dist[a].iter() {
                let next_bit = bit ^ w;
                if !dist[b].contains(&next_bit) {
                    next.push(next_bit);
                }
            }

            if next.is_empty() {
                continue;
            }

            queue.push_back(b);

            for &bit in next.iter() {
                dist[b].insert(bit);
            }
        }
    }

    let ans = dist[n - 1].iter().map(|&v| v).min().unwrap_or(usize::MAX);
    if ans == usize::MAX {
        println!("-1");
        return;
    }
    println!("{}", ans);
}
