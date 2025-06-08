use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut now = 0;
    let mut cnt = 0;
    let mut i = 0;

    while i < n {
        if now + a[i] <= k {
            now += a[i];
            i += 1;
        } else {
            now = 0;
            cnt += 1;
        }
    }

    if now > 0 {
        cnt += 1;
    }

    let ans = cnt;
    println!("{}", ans);
}
