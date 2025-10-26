use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        a: [usize; n],
    }
    let a = a.iter().copied().sorted().collect_vec();
    let run_length = a.iter().copied().dedup_with_count().collect_vec();

    let mut r = 0;
    let mut ans = 0_u64;
    let mut now_count = 0;
    let mut pre_x = run_length[run_length.len() - 1].1;

    for l in 0..run_length.len() {
        while now_count < c {
            now_count += run_length[r % run_length.len()].0;
            r += 1;
        }

        let (num, x) = run_length[l];
        let x = if x <= pre_x { m + x } else { x };
        ans += ((x - pre_x) * now_count) as u64;
        now_count -= num;
        pre_x = x;
    }

    println!("{}", ans);
}
