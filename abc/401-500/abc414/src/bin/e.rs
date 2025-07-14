use ac_library::*;
use itertools::Itertools;
use proconio::{fastout, input};

type MInt = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let ans = MInt::new(n + 1) * n / 2
        - MInt::new(
            std::iter::successors(Some(1), |&i| {
                if i <= n {
                    let y = n / i;
                    Some(n / y + 1)
                } else {
                    None
                }
            })
            .tuple_windows()
            .map(|(i, ni)| {
                let y = n / i;
                (ni - i) * y
            })
            .sum::<u64>(),
        );

    println!("{}", ans);
}
