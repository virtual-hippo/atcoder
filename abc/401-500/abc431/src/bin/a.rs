use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        b: usize,
    }

    let ans = h.saturating_sub(b);
    println!("{}", ans);
}
