use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        qr: [(usize,usize); n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            t: usize,
            d: usize,
        }
        let t = t - 1;
        if d % qr[t].0 == qr[t].1 {
            println!("{}", d);
        } else if d % qr[t].0 < qr[t].1 {
            println!("{}", d + qr[t].1 - d % qr[t].0);
        } else {
            println!("{}", d + qr[t].0 + qr[t].1 - d % qr[t].0);
        }
    }
}
