// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut primes = vec![];
    for i in 1..=n {
        if i * i == n {
            println!("{}", 2 * (i-1));
            return;
        } else if (i-1) * (i-1) < n && n < i * i {
            for j in 1..i {
                if n % j == 0 {
                    primes.push((j, n/j));
                }
            }
            break;
        }
    }
    if primes.len() == 1 {
        println!("{}", n-1);
    } else {
        println!("{}", primes[primes.len()-1].0 - 1 + primes[primes.len()-1].1 - 1);
    }
}

