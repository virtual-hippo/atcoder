use proconio::{fastout, input, marker::*};
use std::{cmp::Reverse, collections::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
    }

    let mut a = vec![0; n];
    let mut tail_set = BTreeSet::new();
    let mut head_set = BTreeSet::new();

    let mut s = 0;

    for i in 0..(n - k) {
        tail_set.insert((Reverse(0), i));
    }
    for i in (n - k)..n {
        head_set.insert((0, i));
    }

    for _ in 0..q {
        input! {
            x: Usize1,
            y: i64,
        }

        let (h_py, h_px) = head_set.first().copied().unwrap();
        let (Reverse(t_py), t_px) = tail_set.first().copied().unwrap_or((Reverse(i64::MIN), usize::MAX));

        if head_set.contains(&(a[x], x)) {
            head_set.remove(&(a[x], x));
            if h_py < y {
                head_set.insert((y, x));
                s += y - a[x];
            } else if y < t_py {
                head_set.insert((t_py, t_px));
                tail_set.remove(&(Reverse(t_py), t_px));
                tail_set.insert((Reverse(y), x));
                s += t_py - a[x];
            } else if t_py <= y && y <= h_py {
                head_set.insert((y, x));
                s += y - a[x];
            }
        } else {
            if h_py < y {
                head_set.remove(&(h_py, h_px));
                tail_set.insert((Reverse(h_py), h_px));

                tail_set.remove(&(Reverse(a[x]), x));
                head_set.insert((y, x));
                s += y - h_py;
            } else {
                tail_set.remove(&(Reverse(a[x]), x));
                tail_set.insert((Reverse(y), x));
            }
        }

        println!("{}", s);
        a[x] = y;
    }
}
