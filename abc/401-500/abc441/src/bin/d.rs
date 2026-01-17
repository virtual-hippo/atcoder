use itertools::*;
use proconio::{fastout, input, marker::*};

fn dfs(to: &Vec<Vec<(usize, usize)>>, s: usize, t: usize, l: usize, depth: usize, cost: usize, ok: &mut [bool], u: usize) {
    if depth == l {
        ok[u] = ok[u] || (s <= cost && cost <= t);
        return;
    }

    for &(v, c) in to[u].iter() {
        dfs(to, s, t, l, depth + 1, cost + c, ok, v);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: usize,
        t: usize,
    }

    let mut to = vec![vec![]; n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
            c: usize,
        }
        to[u].push((v, c));
    }

    let mut ok = vec![false; n];
    dfs(&to, s, t, l, 0, 0, &mut ok, 0);

    let ans = (0..n).filter(|&i| ok[i]).map(|i| i + 1).collect_vec();
    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
