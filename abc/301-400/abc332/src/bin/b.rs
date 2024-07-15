use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    }
    let mut glass = 0;
    let mut cup = 0;
    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if cup == 0 {
            cup = m;
        } else {
            if cup >= (g - glass) {
                cup -= g - glass;
                glass = g;
            } else {
                glass += cup;
                cup = 0;
            }
        }
    }
    println!("{} {}", glass, cup);
}
