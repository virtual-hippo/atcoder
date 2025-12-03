use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let has_zero = s.chars().any(|c| c == '0');

    let ans = if has_zero {
        let v = s.chars().filter(|&c| c != '0').sorted().collect::<Vec<char>>();
        let cnt_zero = s.chars().filter(|&c| c == '0').count();
        format!(
            "{}{}{}",
            v[0],
            (0..cnt_zero).map(|_| '0').collect::<String>(),
            v.iter().skip(1).collect::<String>()
        )
    } else {
        s.chars().sorted().collect::<String>()
    };

    println!("{}", ans);
}
