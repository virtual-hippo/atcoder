use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    }
    let mut swaps = 0;
    for i in 0..n {
        if i < n - 1 && p[i] == i + 1 {
            (p[i], p[i + 1]) = (p[i + 1], p[i]);
            swaps += 1;
        } else if p[i] == i + 1 {
            (p[i], p[i - 1]) = (p[i - 1], p[i]);
            swaps += 1;
        }
    }

    let ans = swaps;
    println!("{}", ans);
}
