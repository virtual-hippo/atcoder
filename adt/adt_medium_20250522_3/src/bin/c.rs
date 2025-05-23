use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut i = n;
    while i <= 919 {
        let a = (i / 100) % 10;
        let b = (i / 10) % 10;
        let c = i % 10;

        if a * b == c {
            println!("{}", i);
            return;
        }
        i += 1;
    }
}
