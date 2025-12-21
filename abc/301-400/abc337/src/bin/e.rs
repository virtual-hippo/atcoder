use itertools::*;
use proconio::{input, marker::Chars, source::line::LineSource};
use std::io::{self, BufReader};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
    }

    let m = (0_usize..).find(|&x| (1 << x) >= n).unwrap();
    println!("{}", m);

    for bit in 0..m {
        let a = (0..n).filter(|&i| (i >> bit) & 1 == 1).map(|i| i + 1).collect_vec();
        print!("{} ", a.len());
        print_vec_1line(&a);
    }
    input! {
        s: Chars,
    }

    let ans = (0..m).filter(|&i| s[i] == '1').fold(0, |acc, i| acc | 1 << i);
    println!("{}", ans + 1);
}

pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
