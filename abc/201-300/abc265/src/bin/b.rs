use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a: [usize; n-1],
        xy: [(usize, usize); m],
    }
    let mut current = (1, t);
    let mut cur = 0;
    for i in 0..n-1 {
        if cur < xy.len() {
            if current.0 == xy[cur].0 {
                current.1 += xy[cur].1;
            }
        }
        if current.1 <= a[i] {
            println!("No");
            return;
        }
        current.1 -= a[i];
        current.0 += 1;
        if cur + 1 < m && current.0 > xy[cur].0 {
            cur += 1;
        }
    }
    println!("Yes");
}

