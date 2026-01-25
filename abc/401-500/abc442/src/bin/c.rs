use itertools::*;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![]; n];

    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        g[a].push(b);
        g[b].push(a);
    }

    let ans = (0..n)
        .map(|i| {
            let c = g[i].len() + 1;
            let v = n.saturating_sub(c);

            let ans = if v < 3 { 0 } else { (v * (v - 1) * (v - 2)) / 6 };
            ans
        })
        .collect_vec();
    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
