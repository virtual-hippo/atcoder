use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    let mut bucket = vec![0; n];
    for i in 0..n {
        if a[i] < n {
            bucket[a[i]] += 1;
        }
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        if bucket[i] == 0 {
            set.insert(i);
        }
    }

    for _ in 0..q {
        input! {
            i: usize,
            x: usize,
        }
        if a[i - 1] < n {
            bucket[a[i - 1]] -= 1;
            if bucket[a[i - 1]] == 0 {
                set.insert(a[i - 1]);
            }
        }

        if x < n {
            bucket[x] += 1;
            if set.contains(&x) {
                set.remove(&x);
            }
        }
        a[i - 1] = x;

        let mex = if let Some(&val) = set.iter().next() {
            val
        } else {
            n
        };
        println!("{}", mex);
    }
}
