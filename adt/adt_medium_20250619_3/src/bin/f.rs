use proconio::{fastout, input, marker::Usize1};
use std::{collections::VecDeque, vec};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut tka = vec![];

    for _ in 0..n {
        input! {
            t: u64,
            k: usize,
            a: [Usize1; k],
        }
        tka.push((t, k, a));
    }

    let mut deq = VecDeque::new();

    let mut studied = vec![false; n];

    let mut ans: u64 = tka[n - 1].0;
    studied[n - 1] = true;

    for &v in tka[n - 1].2.iter() {
        deq.push_back(v);
    }

    while let Some(v) = deq.pop_front() {
        if studied[v] {
            continue;
        }
        studied[v] = true;
        ans += tka[v].0;
        for &u in tka[v].2.iter() {
            deq.push_back(u);
        }
    }

    println!("{}", ans);
}
