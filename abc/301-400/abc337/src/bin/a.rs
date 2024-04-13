use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut t = 0;
    let mut a = 0;
    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }
        t += x;
        a += y;
    }
    if t > a {
        println!("Takahashi");
    } else if a > t {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
