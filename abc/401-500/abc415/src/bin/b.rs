use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let ans = s
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .map(|(i, v)| (i + 1, v))
        .collect::<Vec<_>>();

    for i in 0..ans.len() / 2 {
        println!("{},{}", ans[i * 2].0, ans[i * 2 + 1].0);
    }
}
