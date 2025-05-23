use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        _m: usize,
    }

    let mut p_c_f = vec![];

    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [usize; c],
        }
        let set = f.iter().cloned().collect::<HashSet<_>>();
        p_c_f.push((p, c, set));
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (pi, _ci, fi) = &p_c_f[i];
            let (pj, _cj, fj) = &p_c_f[j];

            let f1 = pi >= pj;
            let f2 = fi.iter().all(|v| fj.contains(v));
            let f3 = pi > pj || fj.iter().any(|v| !fi.contains(v));

            if f1 && f2 && f3 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
