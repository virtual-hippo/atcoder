use itertools::*;
use proconio::{input, source::line::LineSource};
use std::io::{self, BufReader};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
        k: usize,
    }

    let mut f = |x: &[usize]| {
        let x = x.iter().map(|&v| v + 1).collect_vec();
        print!("? ");
        print_vec_1line(&x);
        input! {
            t: usize,
        }
        t
    };

    let mut a = vec![0; n];
    let mut b = vec![0; k + 1];
    let mut t = 0;
    for i in 0..k + 1 {
        let x = (0..k + 1).filter(|&j| i != j).collect_vec();
        b[i] = f(&x);
        t ^= b[i];
    }

    for i in 0..k + 1 {
        a[i] = b[i] ^ t;
    }

    let mut t = 0;
    for i in 0..k - 1 {
        t ^= a[i];
    }
    for i in (k + 1)..n {
        let mut x = (0..k - 1).collect_vec();
        x.push(i);

        a[i] = t ^ f(&x);
    }

    print!("! ");
    print_vec_1line(&a);
}

pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
    let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
    println!("{}", msg);
}
