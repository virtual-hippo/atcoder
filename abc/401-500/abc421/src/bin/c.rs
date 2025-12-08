use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans: u64 = u64::MAX;

    for ch in ['A', 'B'] {
        let mut left = 0;
        let mut right = 0;
        let mut now = 0;

        for i in 0..2 * n {
            if s[i] == ch {
                if left < right {
                    now += right - left;
                }
                left += 1;
            } else {
                if right < left {
                    now += left - right - 1;
                }
                right += 1;
            }
        }
        ans = ans.min(now);
    }

    println!("{}", ans);
}
