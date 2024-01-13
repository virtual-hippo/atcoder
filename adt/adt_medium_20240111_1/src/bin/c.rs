use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut vec = vec![false; n];
    for i in 0..n {
        if vec[i] == false {
            vec[a[i] - 1] = true;
        }
    }
    let ans = vec
        .into_iter()
        .enumerate()
        .filter(|&(_, v)| v == false)
        .map(|(i, _)| i + 1)
        .collect::<Vec<usize>>();

    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}
