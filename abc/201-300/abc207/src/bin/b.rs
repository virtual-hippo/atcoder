use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    if (b - c * d) == 0 {
        println!("-1");
        return;
    }
    let ans = (a * -1) / (b - c * d);
    if ans < 0 {
        println!("-1");
    } else {
        if (a * -1) % (b - c * d) == 0 {
            println!("{}", ans);
        } else {
            println!("{}", ans + 1);
        }
    }
}
