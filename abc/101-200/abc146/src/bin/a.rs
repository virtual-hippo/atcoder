use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let vv = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let i = vv.iter().position(|v| **v == s).unwrap();
    let ans = (14 - i) % 8 + 1;
    println!("{}", ans);
}
