use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let sum = map.iter().map(|(_k, v)| (*v * (*v-1))/2).sum::<usize>();
    for i in 0..n {
        println!("{}", sum-(map.get(&a[i]).unwrap()-1));
    }
}

