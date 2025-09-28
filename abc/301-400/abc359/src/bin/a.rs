use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut cnt = 0;
    for _ in 0..n {
        input! {
            s: String,
        }

        if s == "Takahashi" {
            cnt += 1;
        }
    }

    let ans = cnt;
    println!("{}", ans);
}
