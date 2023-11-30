use proconio::{fastout, input};

fn solve(tka: &Vec<(u64, usize, Vec<usize>)>, pos: usize, skilled: &mut Vec<bool>) -> u64 {
    let mut ret = tka[pos].0;
    skilled[pos] = true;
    for &point in tka[pos].2.iter() {
        if skilled[point - 1] == false {
            ret += solve(tka, point - 1, skilled);
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut tka = Vec::with_capacity(n);
    let mut skilled = vec![false; n];
    for _ in 0..n {
        input! {
            t: u64,
            k: usize,
            a: [usize; k],
        }
        tka.push((t, k, a));
    }
    let ans = solve(&tka, n - 1, &mut skilled);
    println!("{}", ans);
}
