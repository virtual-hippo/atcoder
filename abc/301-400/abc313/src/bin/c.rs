use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let a = a.into_iter().sorted().collect_vec();
    let s = a.iter().sum::<u64>();
    let avg = s / n as u64;
    let m = s % n as u64;

    let goal = {
        let mut goal = vec![avg; n];
        for i in 0..m {
            let j = (n as u64 - i - 1) as usize;
            goal[j] += 1;
        }
        goal
    };

    let ans = (0..n).map(|i| a[i].abs_diff(goal[i])).sum::<u64>() / 2;
    println!("{}", ans);
}
