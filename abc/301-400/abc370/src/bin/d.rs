use proconio::{fastout, input};

// https://stackoverflow.com/questions/48575866/how-to-get-the-lower-bound-and-upper-bound-of-an-element-in-a-btreeset
use std::collections::BTreeSet;
fn neighbors(tree: &BTreeSet<usize>, val: usize) -> (Option<&usize>, Option<&usize>) {
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Excluded(val)));
    let mut after = tree.range((Excluded(val), Unbounded));

    (before.next_back(), after.next())
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut rows = vec![BTreeSet::new(); h];
    let mut cols = vec![BTreeSet::new(); w];

    for i in 0..h {
        for j in 0..w {
            rows[i].insert(j);
            cols[j].insert(i);
        }
    }

    for _ in 0..q {
        input! {
            r: usize,
            c: usize,
        }
        let r = r - 1;
        let c = c - 1;

        if rows[r].contains(&c) {
            rows[r].remove(&c);
            cols[c].remove(&r);
            continue;
        }

        // 横
        {
            let (prev, next) = neighbors(&rows[r], c);
            let ll = *prev.unwrap_or(&c);
            let rr = *next.unwrap_or(&c);
            for i in [ll, rr] {
                rows[r].remove(&i);
                cols[i].remove(&r);
            }
        }

        // 縦
        {
            let (prev, next) = neighbors(&cols[c], r);
            let ll = *prev.unwrap_or(&r);
            let rr = *next.unwrap_or(&r);
            for i in [ll, rr] {
                cols[c].remove(&i);
                rows[i].remove(&c);
            }
        }
    }

    let ans: usize = (0..h).map(|i| rows[i].len()).sum();
    println!("{}", ans);
}
