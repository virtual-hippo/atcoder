use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [usize; n],
    }

    let (now, ans) = a
        .iter()
        .copied()
        .sorted_by_key(|&x| std::cmp::Reverse(x))
        .skip(n - k)
        .fold(
            (0, n - k),
            |(now, ans), a| {
                if now < x {
                    (now + a, ans + 1)
                } else {
                    (now, ans)
                }
            },
        );

    if now >= x {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}
