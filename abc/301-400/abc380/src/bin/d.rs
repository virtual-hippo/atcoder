use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [usize; q],
    }
    let n = s.len();

    for k in k {
        let k = k - 1;
        let leaf = k / n;

        let cnt = solve(0, 1 << 60, leaf);
        let i = k % n;
        if cnt % 2 == 0 {
            print!("{} ", s[i]);
        } else {
            print!("{} ", flip(&s[i]));
        }
    }
}

fn solve(l: usize, r: usize, k: usize) -> usize {
    if l == k {
        return 0;
    }
    let m = (l + r) / 2;
    if k < m {
        solve(l, m, k)
    } else {
        solve(m, r, k) + 1
    }
}

fn flip(c: &char) -> char {
    ((*c as u8) ^ 32) as char
}
