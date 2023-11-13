use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::with_capacity(n);
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        if let Some(val) = map.get_mut(&a[i]) {
            ans += n - *val - i;
            *val -= 1;
            if *val == 0 {
                map.remove(&a[i]);
            }
        }
    }
    println!("{}", ans);
}
