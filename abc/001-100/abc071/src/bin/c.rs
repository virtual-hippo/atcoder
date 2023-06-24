use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let mut vec = map.iter().filter(|(_, &v)| 1 < v).map(|(&k, &v)| (k,v)).collect::<Vec<(u64,u64)>>();
    if vec.len() == 0 {
        println!("0");
        return;
    }
    if vec.len() == 1 && vec[0].1 < 4 {
        println!("0");
        return;
    }

    vec.sort_by(|(a, _), (b, _)| b.cmp(a));
    if vec[0].1 > 4 {
        println!("{}", vec[0].0 * vec[0].0);
    } else {
        println!("{}", vec[0].0 * vec[1].0);
    }
}

