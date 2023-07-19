use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    }
    let mut s = vec![0; n+1];
    let mut set = HashSet::with_capacity(n);
    set.insert(s[0]);
    for i in 0..n {
        s[i+1] = s[i] + a[i];
        set.insert(s[i+1]);
    }
    for v in set.iter() {
        if set.contains(&(v + p)) && set.contains(&(v + p + q)) && set.contains(&(v + p + q + r)) {
            println!("Yes");
            return;
        } 
    }
    println!("No");
}

