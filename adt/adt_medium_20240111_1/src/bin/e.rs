use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = vec![0; 2 * n + 1];
    for i in 0..n {
        ans[2 * (i + 1) - 1] = ans[a[i] - 1] + 1;
        ans[2 * (i + 1)] = ans[a[i] - 1] + 1;
    }
    for i in 0..2 * n + 1 {
        println!("{}", ans[i]);
    }
}
