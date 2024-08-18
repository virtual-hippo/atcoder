use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    }
    let q_max = *(q.iter().max().unwrap());

    let mut a_max = q_max;
    for i in 0..n {
        if a[i] != 0 {
            a_max = a_max.min(q[i] / a[i]);
        }
    }

    let mut ans = 0;
    for a_cnt in 0..=a_max {
        let mut b_max = q_max;
        for i in 0..n {
            if b[i] != 0 {
                b_max = b_max.min((q[i] - a[i] * a_cnt) / b[i]);
            }
        }
        ans = ans.max(a_cnt + b_max);
    }

    println!("{}", ans);
}
