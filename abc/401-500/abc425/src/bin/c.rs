use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut s = vec![0; 2 * n + 1];
    for i in 0..n {
        s[i + 1] = a[i] + s[i];
    }
    for i in n..(2 * n) {
        s[i + 1] = a[i - n] + s[i];
    }

    let mut d = 0;

    for _ in 0..q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {
                    c: usize,
                }
                d += c;
                d %= n;
            },
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let l = l + d;
                let r = r + d;
                let ans = s[r] - s[l - 1];
                println!("{}", ans);
            },
            _ => unreachable!(),
        }
    }
}
