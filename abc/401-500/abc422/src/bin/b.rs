use itertools::*;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let di = [-1, 0, 1, 0];
    let dj = [0, -1, 0, 1];

    let ans = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| s[i][j] == '#')
        .map(|(i, j)| {
            (0..4)
                .filter(|&k| {
                    let di = di[k];
                    let dj = dj[k];
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni < 0 || nj < 0 {
                        return false;
                    }

                    let ni = ni as usize;
                    let nj = nj as usize;

                    if ni >= h || nj >= w {
                        return false;
                    }
                    s[ni][nj] == '#'
                })
                .count()
        })
        .all(|count| count == 2 || count == 4);

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
