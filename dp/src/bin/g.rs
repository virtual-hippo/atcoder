use proconio::{fastout, input};

#[fastout]
fn main() {
    // solve0::solve0();
    solve1::solve1();
}

/// メモ化再帰的に行う解法
pub mod solve0 {
    use super::*;
    pub fn solve0() {
        input! {
            n: usize,
            m: usize,
        }

        let graph = {
            let mut graph = vec![vec![]; n];
            for _ in 0..m {
                input! {
                    u: usize,
                    v: usize,
                }
                let u = u - 1;
                let v = v - 1;
                graph[u].push(v);
            }
            graph
        };

        let mut dp = vec![usize::MAX; n];

        let mut ans = 0;
        for v in 0..n {
            ans = ans.max(rec(&mut dp, &graph, v));
        }

        println!("{}", ans);
    }

    fn rec(dp: &mut Vec<usize>, graph: &Vec<Vec<usize>>, v: usize) -> usize {
        if dp[v] != usize::MAX {
            return dp[v];
        }
        let mut res = 0;
        for &u in graph[v].iter() {
            res = res.max(rec(dp, graph, u) + 1);
        }
        dp[v] = res;
        res
    }
}

pub mod solve1 {
    use super::*;
    use std::collections::VecDeque;

    pub fn solve1() {
        input! {
            n: usize,
            m: usize,
        }

        let (graph, mut deg) = {
            let mut graph = vec![vec![]; n];
            let mut deg = vec![0; n];
            for _ in 0..m {
                input! {
                    u: usize,
                    v: usize,
                }
                let u = u - 1;
                let v = v - 1;
                graph[u].push(v);
                deg[v] += 1;
            }
            (graph, deg)
        };

        let mut dp = vec![0; n];

        let mut que = VecDeque::new();
        for v in 0..n {
            if deg[v] == 0 {
                que.push_back(v);
            }
        }

        while let Some(v) = que.pop_front() {
            for &u in graph[v].iter() {
                deg[u] -= 1;
                if deg[u] > 0 {
                    continue;
                }
                que.push_back(u);
                dp[u] = dp[u].max(dp[v] + 1);
            }
        }

        let mut ans = 0;
        for v in 0..n {
            ans = ans.max(dp[v]);
        }

        println!("{}", ans);
    }
}
