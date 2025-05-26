use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [usize; m],
    }

    let sum_a = a.iter().sum::<usize>();
    if sum_a != n {
        println!("-1");
        return;
    }

    let mut b_map = BTreeMap::new();
    for i in 0..m {
        b_map.insert(x[i], a[i]);
    }

    if !b_map.contains_key(&n) {
        b_map.insert(n, 0);
    }

    let mut l = 0;
    let mut stone = 0;
    let mut ans = 0;

    for (&k, &v) in b_map.iter() {
        let consume = k - l - 1;
        if consume > stone {
            println!("-1");
            return;
        }
        stone -= consume;
        ans += ((consume + 1) * consume) / 2;
        ans += stone * (consume + 1);
        l = k;

        if v > 0 {
            stone += v - 1;
        }
    }

    println!("{}", ans);
}
