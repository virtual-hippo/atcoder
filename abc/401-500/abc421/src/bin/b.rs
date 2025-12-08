use proconio::{fastout, input};

fn f(x: u64, y: u64) -> u64 {
    (x + y).to_string().chars().rev().collect::<String>().parse().unwrap()
}

#[fastout]
fn main() {
    input! {
        x: u64,
        y: u64,
    }

    let mut a = vec![x, y];

    while a.len() < 10 {
        a.push(f(a[a.len() - 1], a[a.len() - 2]));
    }

    let ans = a[9];
    println!("{}", ans);
}
