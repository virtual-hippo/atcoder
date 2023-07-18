use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
    }
    let a = {
        let mut ret = Vec::with_capacity(n);
        for _ in 0..n {
            let mut current = Vec::with_capacity(n);
            input! {
                a: Chars,
            }
            for j in 0..n {
                current.push(a[j] as usize - 48);
            }
            ret.push(current);
        }
        ret
    };

    let mut max = 0;
    for i in 0..n {
        for j in 0..n {
            for k in -1..2 {
                for l in -1..2 {
                    if k == 0 && l == 0 {
                        continue;
                    }
                    let mut current = a[i][j];
                    for m in 1..(n as i32) {
                        let r = (2 * n as i32 + i as i32 + k * m) as usize % n;
                        let c = (2 * n as i32 + j as i32 + l * m) as usize % n;
                        current *= 10;
                        current += a[r][c];
                    }
                    max = max.max(current);
                }
            }
        }
    }
    println!("{}", max);
}
