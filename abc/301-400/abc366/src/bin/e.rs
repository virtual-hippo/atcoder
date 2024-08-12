// マンハッタン距離, 尺取り法
use proconio::{fastout, input};

fn f(src: &Vec<i64>) -> Vec<i64> {
    let len = 4 * 1_000_001;
    let mut ret = vec![0; len];
    let s: i64 = src.iter().sum();
    let mut k = 0;
    let src_len = src.len() as i64;
    ret[0] = s;

    for i in 1..len {
        while k < src_len && src[k as usize] < i as i64 {
            k += 1;
        }
        ret[i] = ret[i - 1] - (src_len as i64 - 2 * k);
    }
    ret.sort();
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
    }
    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            xx: i64,
            yy: i64,
        }
        x.push(xx + 2_000_001);
        y.push(yy + 2_000_001);
    }
    x.sort();
    y.sort();
    let a = f(&x);
    let b = f(&y);

    let mut answer = 0;
    let mut p = 0;
    for i in (0..a.len()).rev() {
        while p < b.len() && a[i] + b[p] <= d {
            p += 1;
        }
        answer += p;
    }
    println!("{}", answer);
}
