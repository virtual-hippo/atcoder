use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut t = 0;
    for i in 0..n {
        let mut hh = h[i];
        t += 3 * (hh / 5);
        hh = hh % 5;
        while hh > 0 {
            t += 1;
            if t % 3 == 0 {
                hh -= 3;
            } else {
                hh -= 1;
            }
        }
    }
    println!("{}", t);
}
