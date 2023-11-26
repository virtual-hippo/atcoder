use proconio::{fastout, input};

fn is_triangle(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> bool {
    let v1 = (p2.0 - p1.0, p2.1 - p1.1);
    let v2 = (p3.0 - p2.0, p3.1 - p2.1);
    v1.0 * v2.1 != v1.1 * v2.0
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if is_triangle(xy[i], xy[j], xy[k]) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
