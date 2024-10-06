use itertools::Itertools;
use proconio::{fastout, input};

fn calc_d(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
    (pos2.0 - pos1.0).pow(2) + (pos2.1 - pos1.1).pow(2)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: f64,
        t: f64,
    }
    let mut abcd = vec![];
    for _ in 0..n {
        input! {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
        }
        abcd.push(((a, b), (c, d)));
    }
    let mut ans = f64::MAX;

    for vec in (0..n).permutations(n) {
        for bit in 0..(1 << n) {
            let mut dir = vec![0; n];
            for i in 0..n {
                if bit & (1 << i) != 0 {
                    dir[i] = 0;
                } else {
                    dir[i] = 1;
                }
            }

            let mut now = (0, 0);
            let mut val = 0.0;
            for i in 0..n {
                let (start, _) = if dir[i] == 0 { (0, 1) } else { (1, 0) };

                let d0 = calc_d(abcd[vec[i]].0, abcd[vec[i]].1);
                let d1 = if start == 0 {
                    calc_d(now, abcd[vec[i]].0)
                } else {
                    calc_d(now, abcd[vec[i]].1)
                };
                if start == 0 {
                    now = abcd[vec[i]].1;
                } else {
                    now = abcd[vec[i]].0;
                }

                val += (d0 as f64).sqrt() / t + (d1 as f64).sqrt() / s;
            }
            ans = ans.min(val);
        }
    }
    println!("{}", ans);
}
