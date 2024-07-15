use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n - 1 {
        input! {
            a: usize,
            b: usize,
            x: usize,
        }
        graph[i].push((i + 1, a));
        graph[i].push((x - 1, b));
    }

    let mut heap = BinaryHeap::new();
    let mut kakutei = vec![false; n];
    let mut costs = vec![usize::MAX; n];
    costs[0] = 0;
    heap.push((Reverse(costs[0]), 0));
    while heap.is_empty() == false {
        let pos = heap.pop().unwrap().1;
        if kakutei[pos] {
            continue;
        }

        kakutei[pos] = true;
        for i in 0..graph[pos].len() {
            let next = graph[pos][i].0;
            let cost = graph[pos][i].1;
            if costs[next] > costs[pos] + cost {
                costs[next] = costs[pos] + cost;
                heap.push((Reverse(costs[next]), next));
            }
        }
    }

    let ans = costs[n - 1];
    println!("{}", ans);
}
