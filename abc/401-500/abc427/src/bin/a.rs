use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let n = s.len();

    let ans = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| i != n / 2)
        .map(|(_, ch)| ch)
        .collect::<String>();
    println!("{}", ans);
}
