use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let ans = s[a..n - b].iter().collect::<String>();
    println!("{}", ans);
}
