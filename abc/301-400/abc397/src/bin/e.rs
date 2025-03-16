// ABC397-E 解説AC
#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use superslice::Ext;

#[fastout]
fn main() {
    solve2::solve();
}

// YouTube による解説: https://www.youtube.com/live/HkDNAOjQTLA
mod solve1 {
    use super::*;
    pub fn solve() {
        input! {
            n: usize,
            k: usize,
        }
        let graph = {
            let mut ret = vec![vec![]; n * k];
            for _ in 0..n * k - 1 {
                input! {
                    a: usize,
                    b: usize,
                }
                ret[a - 1].push(b - 1);
                ret[b - 1].push(a - 1);
            }
            ret
        };

        let mut is_ok = true;

        dfs(&graph, k, usize::MAX, 0, &mut is_ok);
        if is_ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    fn dfs(graph: &Vec<Vec<usize>>, k: usize, parent: usize, v: usize, is_ok: &mut bool) -> usize {
        let mut ret = 1;
        // 次数
        let mut deg = 0;
        for &u in graph[v].iter().filter(|&&u| u != parent) {
            let size = dfs(graph, k, v, u, is_ok);
            // k で割り切れない場合は部分木と接続する必要があるので += 1
            if size % k != 0 {
                deg += 1;
            }
            ret += size;
        }
        if ret % k != 0 {
            deg += 1;
        }
        if deg > 2 {
            *is_ok = false;
        }
        ret
    }
}

// 公式解説: https://atcoder.jp/contests/abc397/editorial/12452
mod solve2 {
    use super::*;
    pub fn solve() {
        input! {
            n: usize,
            k: usize,
        }
        let graph = {
            let mut ret = vec![vec![]; n * k];
            for _ in 0..n * k - 1 {
                input! {
                    a: usize,
                    b: usize,
                }
                ret[a - 1].push(b - 1);
                ret[b - 1].push(a - 1);
            }
            ret
        };

        let mut st = vec![(0, usize::MAX, 0)];
        let mut size = vec![1; n * k];

        while st.len() > 0 {
            let (v, parent, t) = st.pop().unwrap();
            if t == 0 {
                st.push((v, parent, 1));
                for &u in graph[v].iter().filter(|&&u| u != parent) {
                    st.push((u, v, 0));
                }
            } else {
                let mut cnt = 0;
                for &u in graph[v].iter().filter(|&&u| u != parent) {
                    size[v] += size[u];
                    if size[u] > 0 {
                        cnt += 1;
                    }
                }
                if size[v] > k || cnt >= 3 {
                    println!("No");
                    return;
                }
                if size[v] < k && cnt >= 2 {
                    println!("No");
                    return;
                }
                if size[v] == k {
                    size[v] = 0;
                }
            }
        }
        println!("Yes");
    }
}
