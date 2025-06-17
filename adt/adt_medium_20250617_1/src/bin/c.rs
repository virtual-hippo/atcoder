use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let ans = s.iter().filter(|&s| t.iter().any(|t: &String| s.ends_with(t))).count();
    println!("{}", ans);
}
