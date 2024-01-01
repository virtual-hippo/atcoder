use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let ans = 32_u64.pow(a - b);
    println!("{}", ans);
}
