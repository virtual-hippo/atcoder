// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

pub fn prime_factorize(input: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut target = input;
    let mut i = 2;
    while i * i <= target {
        if target % i != 0 {
            i += 1;
            continue;
        }
        let mut e = 0;
        while target % i == 0 {
            e += 1;
            target /= i;
        }
        res.push((i, e));

        i += 1;
    }
    if target != 1 {
        res.push((target, 1));
    }
    res
}

fn main() {
    input! {
        n_ch: Chars,
    }
    let mut sum = 0;
    let mut n = 0;
    for i in 0..n_ch.len() {
        let current = (n_ch[i] as usize) - 48;
        sum += current;
        n += current * 10_usize.pow((n_ch.len() - i - 1) as u32)
    }
    let ret = prime_factorize(n);
    if ret.len() == 1 && ret[0].1 == 1 {
        println!("Prime");
    } else if ret.len() == 0 {
        println!("Not Prime");
    } else if n % 5 != 0 && n % 2 != 0 && sum % 3 != 0 {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}
