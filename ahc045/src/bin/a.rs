#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::source::line::LineSource;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::io::{self, BufReader, Write};
use superslice::Ext;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Square {
    lx: usize,
    rx: usize,
    ly: usize,
    ry: usize,
}
impl Square {
    fn new(lx: usize, rx: usize, ly: usize, ry: usize) -> Self {
        Self { lx, rx, ly, ry }
    }
}

#[derive(Clone)]
struct Input {
    n: usize,            // 都市の個数 n=800
    m: usize,            // 都市のグループの個数 1 <= m <= 400
    q: usize,            // クエリ回数 q=400
    l: usize,            // クエリを行う都市の集合のサイズの上限 3 <= l <= 15
    w: usize, // 都市の座標が含まれる長方形の幅や高さとして有り得る最大値 500 <= w <= 2500
    g: Vec<usize>, // 各グループに割り当てる都市の個数を表す配列 G
    square: Vec<Square>, // 各グループに割り当てる都市の個数を表す配列 G
}

impl Input {
    fn from_stdin() -> Self {
        let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            n: usize, // 都市の個数 n=800
            m: usize, // 都市のグループの個数 1 <= m <= 400
            q: usize, // クエリ回数 q=400
            l: usize, // クエリを行う都市の集合のサイズの上限 3 <= l <= 15
            w: usize, // 都市の座標が含まれる長方形の幅や高さとして有り得る最大値 500 <= w <= 2500
        }
        let mut g = vec![0; m];
        for i in 0..m {
            input! {
                v: usize,
            }
            g[i] = v;
        }
        let mut square = vec![];
        for _ in 0..n {
            input! {
            lx: usize,
            rx: usize,
            ly: usize,
            ry: usize,
                    }
            square.push(Square::new(lx, rx, ly, ry));
        }

        Self {
            n,
            m,
            q,
            l,
            w,
            g,
            square,
        }
    }
}

fn query(c: &[usize]) -> Vec<(usize, usize)> {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    print!("? {}", c.len());
    for &city in c {
        print!(" {}", city);
    }
    println!();
    std::io::stdout().flush().unwrap();
    input! {
        ab: [(usize,usize); c.len()-1], // 都市の組
    }
    ab
}

fn answer(groups: &[Vec<usize>], edges: &[Vec<(usize, usize)>]) {
    println!("!");
    for (i, group) in groups.iter().enumerate() {
        println!(
            "{}",
            group
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        for edge in &edges[i] {
            println!("{} {}", edge.0, edge.1);
        }
    }
    std::io::stdout().flush().unwrap();
}

fn sample(input: &Input) {
    let centers = input
        .square
        .iter()
        .map(|Square { lx, rx, ly, ry }| ((lx + rx) / 2, (ly + ry) / 2))
        .collect::<Vec<(usize, usize)>>();

    let mut cities: Vec<usize> = (0..input.n).collect();
    cities.sort_by_key(|&i| centers[i]);

    let mut groups = Vec::new();
    let mut start_idx = 0;
    for &group_size in &input.g {
        groups.push(cities[start_idx..start_idx + group_size].to_vec());
        start_idx += group_size;
    }

    let mut edges: Vec<Vec<(usize, usize)>> = vec![Vec::new(); input.m];
    for k in 0..input.m {
        for i in (0..input.g[k] - 1).step_by(2) {
            if i < input.g[k] - 2 {
                let ret = query(&groups[k][i..i + 3]);
                edges[k].extend(ret);
            } else {
                edges[k].push((groups[k][i] as usize, groups[k][i + 1] as usize));
            }
        }
    }

    answer(&groups, &edges);
}

#[fastout]
fn main() {
    let input = Input::from_stdin();
    sample(&input);
}
