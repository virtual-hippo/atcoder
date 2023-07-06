use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn as_num(vals: &[&u64]) -> u64 {
    vals.iter()
        .enumerate()
        .fold(0, |sum, (i, &&x)| sum + x * 10_u64.pow(i as u32))
}

fn main() {
    input! {
        n: Chars,
    }
    let vec = n.iter().map(|&x| (x as u64) - 48).collect::<Vec<u64>>();
    let mut ans = 0;
    for perm in vec.iter().permutations(n.len()) {
        for i in 1..n.len() {
            let x = as_num(&perm[0..i]);
            let y = as_num(&perm[i..n.len()]);
            ans = std::cmp::max(ans, x * y);
        }
    }
    println!("{}", ans);
}
