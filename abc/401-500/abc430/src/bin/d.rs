use proconio::{fastout, input};
use std::collections::{BTreeSet, HashMap};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut tree = BTreeSet::new();
    tree.insert(0);
    tree.insert(x[0]);

    let mut map = HashMap::new();
    map.insert(0, x[0].abs_diff(0));
    map.insert(x[0], x[0].abs_diff(0));
    let mut s = x[0].abs_diff(0) * 2;
    println!("{}", s);

    for i in 1..n {
        tree.insert(x[i]);

        if let Some(&y) = tree.range((x[i] + 1)..).next() {
            let old_d = *map.get(&y).unwrap();
            let d = x[i].abs_diff(y);

            map.insert(x[i], d);
            s += d;

            if d < old_d {
                map.insert(y, d);
                s += d;
                s -= old_d;
            }
        }
        if let Some(&y) = tree.range(..x[i]).next_back() {
            let old_d = *map.get(&y).unwrap();
            let d = x[i].abs_diff(y);

            if let Some(&old_d) = map.get(&x[i]) {
                if d < old_d {
                    map.insert(x[i], d);
                    s += d;
                    s -= old_d;
                }
            } else {
                map.insert(x[i], d);
                s += d;
            }

            if d < old_d {
                map.insert(y, d);
                s += d;
                s -= old_d;
            }
        }
        println!("{}", s);
    }
}
