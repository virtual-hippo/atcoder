use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        c: usize,
    }

    let mut x = x;
    let mut ans = 0;
    while x >= 1000 + c {
        ans += 1000;
        x -= 1000 + c;
    }

    println!("{}", ans);
}
