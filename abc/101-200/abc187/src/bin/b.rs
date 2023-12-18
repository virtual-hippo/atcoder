use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let a = (xy[j].1 - xy[i].1) as f64 / (xy[j].0 - xy[i].0) as f64;
            if -1.0 <= a && a <= 1.0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
