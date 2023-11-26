use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut map = BTreeMap::new();
    for i in 0..n {
        map.insert(a[i], 0);
    }

    let mut nokori = k;
    let ko = nokori / n;
    if ko > 0 {
        for i in 0..n {
            if let Some(v) = map.get_mut(&a[i]) {
                *v += ko;
                nokori -= ko;
            }
        }
    }
    for v in map.values_mut() {
        if nokori > 0 {
            *v += 1;
            nokori -= 1;
        } else {
            break;
        }
    }

    for i in 0..n {
        println!("{}", map.get(&a[i]).unwrap());
    }
}
