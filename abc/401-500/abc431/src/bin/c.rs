use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let a = a.iter().copied().sorted().collect_vec();
    let b = b.iter().copied().sorted().collect_vec();

    let mut cnt = 0;
    let mut j = 0;
    for i in 0..n {
        while j < m && a[i] > b[j] {
            j += 1;
        }

        if m <= j {
            break;
        }

        if a[i] <= b[j] {
            j += 1;
            cnt += 1;
        }
    }

    if k <= cnt {
        println!("Yes");
    } else {
        println!("No");
    }
}
