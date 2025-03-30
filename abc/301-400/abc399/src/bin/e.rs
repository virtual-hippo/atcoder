// 解説AC
// functional graph
// 根付木
#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    //_solve0();
    _solve1();
}

// 公式解説
fn _solve0() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    {
        if s == t {
            println!("0");
            return;
        }
        if t.iter()
            .fold(vec![0; 26], |mut acc, &ch| {
                let i = (ch as u8 - b'a') as usize;
                acc[i] = 1;
                acc
            })
            .iter()
            .sum::<usize>()
            == 26
        {
            println!("-1");
            return;
        }
    }

    let to = {
        let mut to = vec![usize::MAX; 26];
        for i in 0..n {
            let u = (s[i] as u8 - b'a') as usize;
            let v = (t[i] as u8 - b'a') as usize;
            if to[u] != usize::MAX && to[u] != v {
                println!("-1");
                return;
            }
            to[u] = v;
        }
        to
    };

    let mut ans = 0;
    // 入次数
    let mut in_deg = vec![0; 26];
    let mut uf = Dsu::new(26);
    for i in (0..26).filter(|&i| to[i] != usize::MAX) {
        if to[i] != i {
            ans += 1;
        }
        in_deg[to[i]] += 1;
        uf.merge(i, to[i]);
    }

    for g in uf.groups().iter() {
        // Functional グラフにおいて連結成分内の頂点の全ての入次数が 1 の場合閉路がある
        let is_cycle = g.iter().all(|&i| in_deg[i] == 1);

        if is_cycle && g.len() > 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

// 解説放送
fn _solve1() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    {
        if s == t {
            println!("0");
            return;
        }
        if t.iter()
            .fold(vec![0; 26], |mut acc, &ch| {
                let i = (ch as u8 - b'a') as usize;
                acc[i] = 1;
                acc
            })
            .iter()
            .sum::<usize>()
            == 26
        {
            println!("-1");
            return;
        }
    }

    let edges = (0..n)
        .map(|i| ((s[i] as u8 - b'a') as usize, (t[i] as u8 - b'a') as usize))
        .collect::<HashSet<(usize, usize)>>();

    let (in_deg, out_deg) = edges.iter().fold(
        (vec![0; 26], vec![0; 26]),
        |(mut in_deg, mut out_deg), &(u, v)| {
            out_deg[u] += 1;
            in_deg[v] += 1;
            (in_deg, out_deg)
        },
    );

    // 出次数が 2 つ以上の貯点があったら不可能
    {
        let is_ok = out_deg.iter().all(|&v| v < 2);
        if !is_ok {
            println!("-1");
            return;
        }
    }

    let mut graph = SccGraph::new(26);
    for &(u, v) in edges.iter() {
        graph.add_edge(u, v);
    }

    let ans = edges.len()
        + graph
            .scc()
            .iter()
            .filter(|g| g.len() > 1)
            .filter(|g| g.iter().all(|&v| in_deg[v] == 1))
            .count()
        - edges.iter().filter(|&&(i, j)| i == j).count();

    println!("{}", ans);
}
