use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: usize,
        a: usize,
        b: usize,
        x: usize,
    }

    let mut ans = 0;
    let mut x = x;
    while x > 0 {
        let mut a = a;
        while x > 0 && a > 0 {
            ans += s;
            x -= 1;
            a -= 1;
        }

        let mut b = b;
        while x > 0 && b > 0 {
            x -= 1;
            b -= 1;
        }
    }

    println!("{}", ans);
}
