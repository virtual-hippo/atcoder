use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        rc: [(i64,i64); n],
    }

    let dh_mx = (0..n).map(|i| rc[i].0).max().unwrap() - (0..n).map(|i| rc[i].0).min().unwrap();
    let dw_mx = (0..n).map(|i| rc[i].1).max().unwrap() - (0..n).map(|i| rc[i].1).min().unwrap();

    let mx = dh_mx.max(dw_mx);

    let ans = if mx % 2 == 0 { mx / 2 } else { mx / 2 + 1 };
    println!("{}", ans);
}
