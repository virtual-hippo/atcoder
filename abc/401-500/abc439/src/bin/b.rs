use proconio::input;
use std::collections::*;

pub fn digits_to_vec(x: usize) -> Vec<usize> {
    if x / 10 == 0 {
        vec![x % 10]
    } else {
        digits_to_vec(x / 10).into_iter().chain(std::iter::once(x % 10)).collect()
    }
}

fn f(n: usize, seen: &mut HashSet<usize>) -> bool {
    if seen.contains(&n) {
        return false;
    }
    seen.insert(n);
    if n == 1 {
        return true;
    }

    let n = digits_to_vec(n).iter().map(|&x| x * x).sum::<usize>();
    if n == 1 {
        true
    } else {
        f(n, seen)
    }
}

fn main() {
    input! {
        n: usize,
    }

    if f(n, &mut HashSet::new()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
