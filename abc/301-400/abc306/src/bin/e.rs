// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;

pub fn binary_search<F: Fn(usize) -> bool>(
    initial_pos: (usize, usize),
    is_ok: F,
) -> (usize, usize) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    (left, right)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
    }
    let mut a = vec![(0, 0); n + 1];
    for i in 0..n+1 {
        a[i].0 = i;
    }
    let mut sorted_a = a.clone();
    let mut idx_set = HashSet::new();
    for i in 1..=k {
        idx_set.insert(i);
    }
    let mut current_sum = 0;

    for _qi in 0..q {
        input! {
            x: usize,
            y: u64,
        }
        let prev_val = a[x];
        let prev_tail = sorted_a[k-1];

        a[x].1 = y;
        sorted_a = a.clone();
        sorted_a.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_a.sort_by(|a, b| b.1.cmp(&a.1));

        if idx_set.contains(&x) && y <= prev_tail.1 {
            current_sum += sorted_a[k-1].1;
            current_sum -= prev_val.1;
            idx_set.remove(&x);
            idx_set.insert(sorted_a[k-1].0);

        } else {
            if prev_tail.1 < y {
                current_sum += y;
                current_sum -= prev_tail.1;
            } else {
            }
        }
        println!("{}", current_sum);
    }
}
