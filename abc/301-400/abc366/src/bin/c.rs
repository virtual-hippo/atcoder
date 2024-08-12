use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut map = HashMap::new();

    for _ in 0..q {
        input! {
            qq: usize,
        }
        match qq {
            1 => {
                input! {
                    x: usize,
                }
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                }
                if let Some(v) = map.get_mut(&x) {
                    *v -= 1;
                    if *v == 0 {
                        map.remove(&x);
                    }
                }
            }
            3 => {
                println!("{}", map.len())
            }
            _ => unreachable!(),
        }
    }
}
