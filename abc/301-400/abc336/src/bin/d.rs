use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dl = vec![0; n];
    dl[0] = 1;
    for i in 1..n {
        dl[i] = (dl[i - 1] + 1).min(a[i]);
    }

    let mut dr = vec![0; n];
    dr[n - 1] = 1;
    for i in (0..n - 1).rev() {
        dr[i] = (dr[i + 1] + 1).min(a[i]);
    }

    let mut ans = 1;
    for i in 0..n {
        ans = ans.max(dl[i].min(dr[i]));
    }
    println!("{}", ans);
}
