use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        a: [u64; n],
        x: u64,
    }
    let s = a.iter().sum::<u64>();
    let left = (x / s) * n;
    let right = {
        let m = x % s;
        let mut current = 0;
        let mut i = 0;
        while current <= m {
            current += a[i];
            i += 1;
        }
        i as u64
    };
    let ans = left + right;
    println!("{}", ans);
}
