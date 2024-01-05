use std::collections::BTreeMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = BTreeMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;
    let mut pre2 = (usize::MAX, 0);
    let mut pre1 = (usize::MAX, 0);
    for (&k, &v) in map.iter() {
        ans = ans.max(v);
        if pre1.0 != usize::MAX && k - pre1.0 == 1 {
            ans = ans.max(v + pre1.1);
            if pre2.0 != usize::MAX && pre1.0 - pre2.0 == 1 {
                ans = ans.max(v + pre1.1 + pre2.1);
            }
        }
        pre2 = pre1;
        pre1 = (k, v);
    }

    println!("{}", ans);
}
