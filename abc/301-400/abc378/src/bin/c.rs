use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    let mut b = vec![];

    for (i, v) in a.iter().enumerate() {
        let i = i + 1;
        if let Some(vv) = map.get_mut(&v) {
            b.push(*vv as i64);
            *vv = i;
        } else {
            map.insert(v, i);
            b.push(-1);
        }
    }
    for v in b.iter() {
        print!("{} ", *v);
    }
}
