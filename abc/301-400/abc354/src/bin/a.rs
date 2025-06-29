use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
    }

    let mut ans = 0;
    let mut v = 0;

    while v <= h {
        v += 1 << ans;
        ans += 1;
    }

    println!("{}", ans);
}
