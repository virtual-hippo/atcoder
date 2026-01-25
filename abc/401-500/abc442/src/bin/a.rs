use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let ans = s.chars().filter(|&ch| ch == 'i' || ch == 'j').count();
    println!("{}", ans);
}
