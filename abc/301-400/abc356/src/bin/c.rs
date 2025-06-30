use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: u32,
    }

    let mut ar = vec![];

    for _ in 0..m {
        input! {
            c: usize,
            xa: [usize; c],
            r: char,
        }
        let mut a = 0;
        for &x in xa.iter() {
            a |= 1 << (x - 1);
        }
        ar.push((a, r));
    }

    let ans = (0_usize..(1 << n))
        .filter(|&bit| {
            (0..m).all(|i| {
                let ok_keys = (ar[i].0 & bit).count_ones();
                if ok_keys < k && ar[i].1 == 'o' {
                    false
                } else if ok_keys >= k && ar[i].1 == 'x' {
                    false
                } else {
                    true
                }
            })
        })
        .count();

    println!("{}", ans);
}
