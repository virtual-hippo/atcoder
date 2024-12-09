use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    for i in 0..=n {
        let mut tmp: u8 = 11;
        for j in 1..10 {
            if n % j != 0 {
                continue;
            }
            if i % (n / j) == 0 {
                tmp = j as u8;
                break;
            }
        }
        if tmp == 11 {
            print!("-");
        } else {
            print!("{}", tmp);
        }
    }
}
