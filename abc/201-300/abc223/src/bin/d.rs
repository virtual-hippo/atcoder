use proconio::{fastout, input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![]; n];
    let mut set = HashSet::new();
    let mut cnt = vec![0; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        graph[a].push(b);
        cnt[b] += 1;
        set.insert(a);
    }
    for i in 0..n {
        graph[i].sort();
    }

    let mut heap = (0..n)
        .filter(|&i| cnt[i] == 0)
        .map(|i| Reverse(i))
        .collect::<BinaryHeap<_>>();
    let mut ans = vec![];

    while let Some(Reverse(i)) = heap.pop() {
        ans.push(i);
        for &to in graph[i].iter() {
            cnt[to] -= 1;
            if cnt[to] == 0 {
                heap.push(Reverse(to));
            }
        }
    }
    if ans.len() != n {
        println!("{}", -1);
        return;
    }

    let mut iter = ans.iter();
    print!("{}", iter.next().unwrap() + 1);
    for val in iter {
        print!(" {}", val + 1);
    }
}
