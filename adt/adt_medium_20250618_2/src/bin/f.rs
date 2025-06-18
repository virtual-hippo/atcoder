use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
        n: usize,
        s: [String; n],
    }

    let order = {
        let mut order = vec![0; 26];
        for (i, c) in x.chars().enumerate() {
            order[c as usize - 'a' as usize] = i;
        }
        order
    };

    let ans = s
        .iter()
        .sorted_by_key(|ss| {
            ss.chars()
                .enumerate()
                .map(|(i, c)| (order[c as usize - 'a' as usize] + 1) * 27_usize.pow((10 - i - 1) as u32))
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
