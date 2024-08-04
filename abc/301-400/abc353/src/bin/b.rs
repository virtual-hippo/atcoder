use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut s = 0;
    let mut ans = 0;
    for i in 0..n {
        if s + a[i] <= k {
            s += a[i];
        } else {
            ans += 1;
            s = a[i];
        }
    }
    if s != 0 {
        ans += 1;
    }
    println!("{}", ans);
}
