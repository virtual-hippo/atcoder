use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        current: (usize, usize),
        c: [Chars; h],
        x: Chars,
    }
    let mut cur = (current.0 - 1, current.1 - 1);
    for &xx in x.iter() {
        if xx == 'U' {
            if cur.0 > 0 && c[cur.0 - 1][cur.1] == '.' {
                cur.0 -= 1;
            }
        }
        if xx == 'D' {
            if cur.0 < h - 1 && c[cur.0 + 1][cur.1] == '.' {
                cur.0 += 1;
            }
        }
        if xx == 'L' {
            if cur.1 > 0 && c[cur.0][cur.1 - 1] == '.' {
                cur.1 -= 1;
            }
        }
        if xx == 'R' {
            if cur.1 < w - 1 && c[cur.0][cur.1 + 1] == '.' {
                cur.1 += 1;
            }
        }
    }
    println!("{} {}", cur.0 + 1, cur.1 + 1);
}
