use itertools::*;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let dp = (1..32).fold(vec![a.clone()], |mut dp, _| {
        let pre = dp.len() - 1;
        let new = (0..n).map(|i| dp[pre][dp[pre][i]]).collect_vec();
        dp.push(new);
        dp
    });

    let ans = (0..n).map(|i| dp[30][i] + 1).collect_vec();
    print_vec_1line(&ans);
}

// ------------------------------------------------------------------------------------------------
// libs
// ------------------------------------------------------------------------------------------------
fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
