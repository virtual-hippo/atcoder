use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, ch)| ch)
        .collect::<String>();
    println!("{}", ans);
}
