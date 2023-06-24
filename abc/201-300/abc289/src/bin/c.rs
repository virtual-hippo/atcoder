use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut s = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            a: [usize; c],
        }
        s.push(a);
    }
    let mut ans = 0;

    for bit in 0..(1<<m) {
        let mut set = HashSet::new();
        for i in 0..m {
            if bit & (1 << i) == 0 {
                // 選ぶ
                for &j in s[i].iter() {
                    set.insert(j);
                }
            } else {
                // 選ばない
            }
        }
        if set.len() == n {
            ans += 1;
        }
    }
    println!("{}", ans);
}

