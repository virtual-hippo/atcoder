use itertools::*;
use proconio::{fastout, input};

fn dfs(depth: usize, n: usize, parent: usize, b: &mut Vec<usize>) {
    if depth == n {
        b.push(parent);
        return;
    }

    let l = parent / 2;
    let r = parent / 2 + parent % 2;
    dfs(depth + 1, n, l, b);
    dfs(depth + 1, n, r, b);
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut b = vec![];
    dfs(0, n, k, &mut b);

    let unique_cnt = b.iter().copied().unique().count();

    println!("{}", unique_cnt - 1);
    print_vec_1line(&b);
}

#[fastout]
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
