use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: i64,
    }
    let mut x = 0;
    let mut ans = i64::MAX;
    while x * x <= d {
        let y = ((d - x * x) as f64).sqrt() as i64;
        for yy in y - 3..y + 4 {
            ans = ans.min((x * x + yy * yy - d).abs());
        }
        x += 1;
    }
    println!("{}", ans);
}
