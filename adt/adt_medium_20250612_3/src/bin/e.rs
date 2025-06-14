use proconio::{fastout, input};
use std::collections::{HashMap, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut que = VecDeque::new();
    let mut ans = 0;
    que.push_back(vec![(n, 1)]);

    while let Some(vals) = que.pop_front() {
        ans += vals.iter().map(|&(v, c)| v * c).sum::<usize>();
        let mut map = HashMap::new();

        for &(x, c) in &vals {
            if x % 2 == 0 {
                *map.entry(x / 2).or_insert(0) += 2 * c;
            } else {
                *map.entry(x / 2).or_insert(0) += c;
                *map.entry(x / 2 + 1).or_insert(0) += c;
            }
        }

        let new = map.into_iter().filter(|&(x, _)| x > 1).collect::<Vec<_>>();
        if new.len() == 0 {
            continue;
        }

        que.push_back(new);
    }

    println!("{}", ans);
}
