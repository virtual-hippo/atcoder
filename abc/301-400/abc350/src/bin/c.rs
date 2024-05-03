use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ind = vec![0; n];
    for i in 0..n {
        ind[a[i] - 1] = i;
    }
    let mut a = a;
    let mut ans = vec![];
    for i in 0..n {
        if a[i] != i + 1 {
            ans.push((ind[a[i] - 1] + 1, ind[i] + 1));
            let old_a = (a[ind[i]], a[i]);
            let old_ind = (ind[a[i] - 1], ind[i]);
            let vv = a[i] - 1;
            (a[i], a[ind[i]]) = old_a;
            ind[vv] = old_ind.1;
            ind[i] = i;
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{} {}", ans[i].0, ans[i].1);
    }
}
