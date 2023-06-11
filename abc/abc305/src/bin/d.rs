use proconio::input;
use std::collections::HashMap;

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
        q: usize,
    }
    let mut map = HashMap::new();
    map.insert(0,0);
    let mut prev = 0;
    for i in 1..n/2+1 {
        let current = a[2*i] - a[2*i-1] + prev;
        map.insert(a[2*i], current);
        prev = current;
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let mut ans = 0;
        {
            let l_is_ok = |x: usize| a[x] <= l;
            let l_result = binary_search((0, a.len()), l_is_ok);
            if l_result.1 % 2 == 0 {
                ans += (a[l_result.1] - l) as i64;
                ans -= *(map.get(&a[l_result.1]).unwrap()) as i64;
            } else {
                ans -= *(map.get(&a[l_result.0]).unwrap()) as i64;
            }
        }
        {
            let r_is_ok = |x: usize| a[x] <= r;
            let r_result = binary_search((0, a.len()), r_is_ok);
            if r_result.0 % 2 == 1 {
                ans += (r - a[r_result.0]) as i64;
                ans += *(map.get(&a[r_result.0 - 1]).unwrap()) as i64;
            } else {
                ans += *(map.get(&a[r_result.0]).unwrap()) as i64;
            }
        }
        
        println!("{:?}", ans);
        
    }
}

