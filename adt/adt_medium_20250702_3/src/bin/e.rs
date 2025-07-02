use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let yes = s
        .chars()
        .fold((HashMap::from([((0, 0), 1)]), (0, 0)), |(mut map, now), c| {
            let now = match c {
                'D' => (now.0, now.1 - 1),
                'U' => (now.0, now.1 + 1),
                'L' => (now.0 - 1, now.1),
                'R' => (now.0 + 1, now.1),
                _ => unreachable!(),
            };
            *map.entry(now).or_insert(0) += 1;
            (map, now)
        })
        .0
        .values()
        .any(|&x| x > 1);

    if yes {
        println!("Yes");
    } else {
        println!("No");
    }
}
