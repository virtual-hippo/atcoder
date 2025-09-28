use proconio::{fastout, input};
use rustc_hash::FxHashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut cnt = vec![0; n + 1];

    let mut set = FxHashSet::default();

    for i in 1..n + 1 {
        set.insert(i as i32);
    }
    for v in a.iter() {
        if *v == -1 {
            continue;
        }
        cnt[*v as usize] += 1;
        set.remove(v);
    }

    if cnt.iter().any(|&x| x > 1) {
        println!("No");
        return;
    }

    let mut vals = set.into_iter().collect::<Vec<_>>();

    let mut p = a.clone();
    for i in 0..n {
        if p[i] != -1 {
            continue;
        }
        let v = vals.pop().unwrap();
        p[i] = v;
    }

    println!("Yes");
    for i in 0..n {
        if i == n - 1 {
            print!("{}", p[i]);
        } else {
            print!("{} ", p[i]);
        }
    }
}
