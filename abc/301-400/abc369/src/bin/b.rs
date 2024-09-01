use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let (mut l, mut r) = (1000, 1000);
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: i32,
            s: char,
        }
        if s == 'L' {
            if l != 1000 {
                ans += (l - a).abs();
            }
            l = a;
        } else {
            if r != 1000 {
                ans += (r - a).abs();
            }
            r = a;
        }
    }
    println!("{}", ans);
}
