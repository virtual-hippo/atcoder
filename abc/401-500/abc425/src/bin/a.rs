use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        ans += (-1_i64).pow((i) as u32) * i.pow(3) as i64;
    }

    println!("{}", ans);
}
