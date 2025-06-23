use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64;n],
    }

    let mut ans = 0;

    for i in 0..n {
        ans += a[i];
        if ans < 0 {
            ans = 0;
        }
    }

    println!("{}", ans);
}
