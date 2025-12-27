use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
        f: usize,
    }

    let mut n = f;

    while n <= d {
        n += 7;
    }

    let ans = n - d;
    println!("{}", ans);
}
