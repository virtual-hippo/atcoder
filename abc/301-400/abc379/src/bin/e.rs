use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut d = vec![0; n];

    for i in 0..n {
        let num = s[i].to_digit(10).unwrap() as u64;
        if i == 0 {
            d[i] = num;
        } else {
            d[i] = d[i - 1] + (i as u64 + 1) * num;
        }
    }
    let mut d: Vec<u64> = d.into_iter().rev().collect();

    for i in 0..n - 1 {
        let carry = d[i] / 10;
        d[i] = d[i] % 10;
        d[i + 1] += carry;
    }

    for i in (0..n).rev() {
        print!("{}", d[i]);
    }
}
