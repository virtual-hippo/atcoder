use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a = vec![vec![]; n];

    for i in 0..n {
        input! {
            l: usize,
        }

        for _ in 0..l {
            input! {
                x: usize,
            }
            a[i].push(x);
        }
    }

    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        }
        let s = s - 1;
        let t = t - 1;
        println!("{}", a[s][t]);
    }
}
