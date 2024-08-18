use proconio::{fastout, input};

fn f(n: usize, x: usize, nx: usize, s: &mut Vec<i64>) {
    let (l, r) = (x.min(nx), x.max(nx));

    if r - l < n - (r - l) {
        let v = (n - 2 * (r - l)) as i64;
        s[l] += v;
        s[r] -= v;
    } else {
        let v = (2 * (r - l) - n) as i64;
        s[r] += v;
        // s[n - 1] -= v;
        s[0] += v;
        s[l] -= v;
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [usize; m],
    }
    for i in 0..m {
        x[i] -= 1;
    }

    let origin = {
        let mut ret = 0;
        for i in 1..m {
            let (r, l) = (x[i - 1].max(x[i]), x[i].min(x[i - 1]));
            ret += (r - l).min(n - (r - l));
        }
        ret as i64
    };

    let mut s = vec![0_i64; n];
    for i in 1..m {
        f(n, x[i - 1], x[i], &mut s);
    }

    for i in 0..n - 1 {
        s[i + 1] += s[i];
    }

    let min = (0..n).map(|i| s[i]).min().unwrap();
    let ans = origin + min;
    println!("{}", ans);
}
