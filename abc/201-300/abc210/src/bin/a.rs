use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    }
    let ans = (1..n + 1)
        .map(|i| if i <= a { x } else { y })
        .sum::<usize>();
    println!("{}", ans);
}
