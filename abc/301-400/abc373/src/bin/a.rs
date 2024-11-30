use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 12],
    }
    let ans = s
        .iter()
        .enumerate()
        .filter(|(i, v)| v.len() == i + 1)
        .count();
    println!("{}", ans);
}
