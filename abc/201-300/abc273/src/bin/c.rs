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
        a: [usize; n],
    }
    let mut set =  HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
    }
    let mut sorted_a = set.iter().map(|&v|v).collect::<Vec<usize>>();
    sorted_a.sort();
    let mut cnt = vec![0; n];

    for &target in a.iter() {
        let is_ok = |x: usize| sorted_a[x] <= target;
        let result = binary_search((0, sorted_a.len()), is_ok);
        cnt[sorted_a.len()-result.1] += 1;
    }

    for k in 0..n {
        println!("{}", cnt[k]);
    }
}

