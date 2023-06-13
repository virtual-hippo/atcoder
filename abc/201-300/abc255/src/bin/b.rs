use proconio::input;

fn calc_d(pos1: (i64, i64), pos2: (i64, i64)) -> i64 {
    (pos2.0 - pos1.0).pow(2) + (pos2.1 - pos1.1).pow(2) 
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(i64, i64); n],
    }
    let mut people = vec![false; n+1];
    for i in 0..k {
        people[a[i]] = true;
    }
    let max = (1..n+1).map(|i|{
        if people[i] {
            0
        } else {
            a.iter().map(|j| calc_d(xy[i-1], xy[j-1])).min().unwrap()
        }
    }).max().unwrap();
    println!("{}", (max as f64).sqrt());
}

