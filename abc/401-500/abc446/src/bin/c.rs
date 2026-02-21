use proconio::input;
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        map.insert(i, a[i]);

        let mut pc = b[i];
        loop {
            let (k, v) = map.pop_first().unwrap();

            if pc < v {
                map.insert(k, v - pc);
                break;
            } else if v == pc {
                break;
            } else {
                pc -= v;
            }
        }

        if i >= d {
            map.remove(&(i.saturating_sub(d)));
        }
    }

    let ans: usize = map.values().sum();
    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}
