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
        (w,h): (usize, usize),
        n: usize,
        pq: [(usize, usize); n],
        an: usize,
        a_in: [usize; an],
        bn: usize,
        b_in: [usize; bn],
    }
    let mut a = vec![0];
    a.extend(a_in);
    a.push(w);
    let mut b = vec![0];
    b.extend(b_in);
    b.push(h);

    let mut map = HashMap::new();

    for i in 0..n {
        let x = {
            let result = {
                let is_ok = |x: usize| a[x] < pq[i].0;
                binary_search((0, a.len()), is_ok)
            };
            a[result.1]
        };
        let y = {
            let result = {
                let is_ok = |x: usize| b[x] < pq[i].1;
                binary_search((0, b.len()), is_ok)
            };
            b[result.1]
        };

        *map.entry((x,y)).or_insert(0) += 1;
    }
    let max = map.iter().fold(0, |ret, (_ , val)| std::cmp::max(ret, *val));
    let min = if map.len() < (an+1) * (bn+1) {
        0
    } else {
        map.iter().fold(n, |ret, (_ , val)| std::cmp::min(ret, *val))
    };
    println!("{} {}", min, max);
}
