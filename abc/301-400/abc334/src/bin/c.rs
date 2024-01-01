use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        k: usize,
        a: [usize; k],
    }
    if k == 1 {
        println!("0");
        return;
    }
    if k % 2 == 0 {
        let ans = (0..k / 2).map(|i| a[2 * i + 1] - a[2 * i]).sum::<usize>();
        println!("{}", ans);
        return;
    }
    let koho1 = (0..k / 2)
        .map(|i| a[2 * i + 1] - a[2 * i])
        .collect::<Vec<usize>>();
    let koho2 = (0..k / 2)
        .map(|i| a[2 * i + 2] - a[2 * i + 1])
        .collect::<Vec<usize>>();
    let mut s1 = vec![0; koho1.len() + 1];
    let mut s2 = vec![0; koho2.len() + 1];
    for i in 0..koho1.len() {
        s1[i + 1] = s1[i] + koho1[i];
        s2[i + 1] = s2[i] + koho2[i];
    }

    let origin = koho1.iter().sum::<usize>();
    let mut ans = origin;
    for i in 0..koho1.len() {
        let current = s1[i] + (s2[koho1.len()] - s2[i]);
        ans = ans.min(current)
    }
    println!("{}", ans);
}
