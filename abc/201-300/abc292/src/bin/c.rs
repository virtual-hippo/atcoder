use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    const M: usize = 200_005;
    let mut divs = vec![vec![]; M];
    for i in 1..=M {
        let mut j = i * 2;
        while j < M {
            divs[j].push(i);
            j += i;
        }
    }
    for i in 1..M {
        divs[i].push(i);
    }

    let cnt = (0..n).map(|i| divs[i].len() as u64).collect_vec();

    let ans = (1..n).map(|i| cnt[i] * cnt[n - i]).sum::<u64>();
    println!("{}", ans);
}
