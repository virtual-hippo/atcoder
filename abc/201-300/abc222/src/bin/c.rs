use proconio::marker::Chars;
use proconio::{fastout, input};

pub const KACHI: usize = 0;
pub const MAKE: usize = 1;
pub const AIKO: usize = 2;

fn janken(jibun: char, aite: char) -> usize {
    if jibun == aite {
        return AIKO;
    }
    if jibun == 'G' && aite == 'C' {
        return KACHI;
    }
    if jibun == 'C' && aite == 'P' {
        return KACHI;
    }
    if jibun == 'P' && aite == 'G' {
        return KACHI;
    }
    MAKE
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2*n],
    }
    let mut rank = (0..2 * n).collect::<Vec<usize>>();
    let mut shosu = vec![0; 2 * n];
    for i in 0..m {
        for j in 0..n {
            let left = rank[j * 2];
            let right = rank[j * 2 + 1];
            if janken(a[left][i], a[right][i]) == KACHI {
                shosu[left] += 1;
            } else if janken(a[left][i], a[right][i]) == MAKE {
                shosu[right] += 1;
            }
        }
        rank.sort_by(|a, b| match shosu[*b].cmp(&shosu[*a]) {
            std::cmp::Ordering::Equal => a.cmp(&b),
            other => other,
        });
    }
    for i in 0..2 * n {
        println!("{}", rank[i] + 1);
    }
}
