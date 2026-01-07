use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let ans = c.saturating_sub(a - b);
    println!("{}", ans);
}
