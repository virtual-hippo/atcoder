use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let s = {
        let mut s = vec![0; n];
        for i in 0..n {
            input! {
                a: Chars,
            }
            s[i] = a.iter().fold(0, |mut acc, &c| {
                acc <<= 1;
                if c == 'o' {
                    acc |= 1;
                }
                acc
            });
        }
        s
    };

    let mut ans = m;
    for bit in 0..1 << n {
        let mut now = 0;
        let mut cnt = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                now |= s[i];
                cnt += 1;
            }
        }

        if now == (1 << m) - 1 {
            ans = ans.min(cnt);
        }
    }

    println!("{}", ans);
}
