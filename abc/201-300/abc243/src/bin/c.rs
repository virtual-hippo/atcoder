use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xy: [(i64,i64); n],
        s: Chars,
    }
    let mut map: HashMap<i64, (i64, i64)> = HashMap::with_capacity(n);
    for i in 0..n {
        let (x, y) = xy[i];
        if let Some(val) = map.get_mut(&y) {
            if s[i] == 'L' {
                (*val).1 = std::cmp::max(val.1, x);
            } else {
                (*val).0 = std::cmp::min(val.0, x);
            };
        } else {
            let val = if s[i] == 'L' { (std::i64::MAX, x) } else { (x, -1) };
            map.insert(y, val);
        }
    }

    let cnt = map
        .values()
        .filter(|&&val| val.0 < val.1 && val.0 != std::i64::MAX && val.1 != -1)
        .count();
    if 0 < cnt {
        println!("Yes");
    } else {
        println!("No");
    }
}
