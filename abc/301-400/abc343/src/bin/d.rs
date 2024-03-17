use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
    }

    let mut state = vec![0; n];
    let mut map = HashMap::new();
    map.insert(0, n);
    for _ in 0..t {
        input! {
            a: usize,
            b: usize,
        }
        if let Some(v) = map.get_mut(&state[a - 1]) {
            *v -= 1;
            if *v == 0 {
                map.remove(&state[a - 1]);
            }
        }
        state[a - 1] += b;
        *map.entry(state[a - 1]).or_insert(0) += 1;
        println!("{}", map.len());
    }
}
