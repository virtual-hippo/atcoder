#![allow(unused_imports)]
use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = vec![];
    for v in 1..=m - 10 {
        let mut now = vec![v];
        recursive(n, m, &mut now, v, &mut ans);
    }

    println!("{}", ans.len());
    for i in 0..ans.len() {
        for j in 0..ans[i].len() {
            print!("{} ", ans[i][j]);
        }
        println!("");
    }
}

fn recursive(n: usize, m: usize, now: &mut Vec<usize>, v: usize, ans: &mut Vec<Vec<usize>>) {
    if now.len() == n {
        ans.push(now.clone());
        return;
    }

    if v + 10 > m {
        return;
    }

    for vv in (v + 10)..=((m - 10 * (n - now.len() - 1)).max(v + 10)) {
        if vv > m {
            return;
        }
        now.push(vv);
        recursive(n, m, now, vv, ans);
        now.pop();
    }
}

mod solve2 {
    fn f(a: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>, n: usize, m: usize) {
        if a.len() == n {
            ans.push(a.clone());
            return;
        }
        let l = if a.len() == 0 { 1 } else { a[a.len() - 1] + 10 };

        if l >= m {
            return;
        }

        let r = m - 10 * (n - a.len() - 1);

        for v in l..r {
            a.push(v);
            f(a, ans, n, m);
            a.pop();
        }
    }

    fn solve2() {
        input! {
            n: usize,
            m: usize,
        }

        let mut ans = vec![];
        f(&mut vec![], &mut ans, n, m + 1);

        println!("{}", ans.len());
        ans.iter().for_each(|v| print_vec_1line(v));
    }

    // ------------------------------------------------------------------------------------------------
    // libs
    // ------------------------------------------------------------------------------------------------
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
}
