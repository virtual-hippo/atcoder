use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ab: [(usize, Usize1); n],
        m: usize,
        s: [Chars; m],
    }

    // 長さ i , かつ j 文字目が k のものがあるかどうか
    let mut bucket = vec![vec![vec![false; 26]; 12]; 12];
    for i in 0..m {
        let s = &s[i];
        for j in 0..s.len() {
            let ch = (s[j] as u8 - b'a') as usize;
            bucket[s.len()][j][ch] = true;
        }
    }

    for j in 0..m {
        let s = &s[j];
        if s.len() != n {
            println!("No");
            continue;
        }
        let ans = (0..n).all(|i| {
            let (a, b) = ab[i];
            let c = (s[i] as u8 - b'a') as usize;
            bucket[a][b][c]
        });

        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
