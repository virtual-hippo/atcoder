use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut k = 0;

    while 2_u64.pow(k) <= n {
        k += 1;
    }
    k -= 1;

    let ans = k;
    println!("{}", ans);
}
