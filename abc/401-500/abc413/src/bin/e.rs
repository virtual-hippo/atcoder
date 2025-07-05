use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        _solve2();
    }
}

//
// swap で解く (解説放送)
// https://www.youtube.com/live/NRb4TT78L8c
//
fn _solve2() {
    input! {
        n: usize,
        p: [u64; 1 << n],
    }

    let mut p = p;

    let mut w = 1;
    while w < (1 << n) {
        let mut l = 0;
        while l < (1 << n) {
            if p[l] > p[l + w] {
                for i in 0..w {
                    p.swap(l + i, l + w + i);
                }
            }
            l += w * 2;
        }
        w <<= 1;
    }

    for (i, v) in p.iter().enumerate() {
        if i == 0 {
            print!("{}", v);
        } else {
            print!(" {}", v);
        }
    }
    println!();
}

fn _solve() {
    input! {
        n: usize,
        p: [u64; 1 << n],
    }

    let mut p = p;
    rec(&mut p, 0, 1 << n);

    for (i, v) in p.iter().enumerate() {
        if i == 0 {
            print!("{}", v);
        } else {
            print!(" {}", v);
        }
    }
    println!();
}

fn rec(p: &mut Vec<u64>, l: usize, r: usize) {
    if r - l < 2 {
        return;
    }

    let m = (l + r) >> 1;

    let mn_l = p[l..m].iter().min().unwrap();
    let mn_r = p[m..r].iter().min().unwrap();

    if mn_l > mn_r {
        p[l..r].reverse();
    }

    rec(p, l, m);
    rec(p, m, r);
}
