use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    }
    let mut i = a;
    while i <= b {
        print!("{} ", i);
        i += d;
    }
}
