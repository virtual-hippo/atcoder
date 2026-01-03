use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![];

    for i in 0..n {
        let a = a[i];
        let i = dp.lower_bound(&a);

        if i == dp.len() {
            dp.push(a);
        } else {
            dp[i] = a;
        }
    }

    println!("{}", dp.len());
}
