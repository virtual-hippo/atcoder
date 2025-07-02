use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    if n >= 0 {
        println!("{}", n / 10);
    } else {
        let n = n.abs();
        if n % 10 == 0 {
            println!("-{}", n / 10);
            return;
        } else {
            println!("-{}", n / 10 + 1);
            return;
        }
    }
}
