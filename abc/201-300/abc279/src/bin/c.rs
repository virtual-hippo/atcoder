// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        (h,w): (usize, usize),
        s: [Chars; h],
        t: [Chars; h],
    }
    let mut s_board = vec![vec![' '; h]; w];
    let mut t_board = vec![vec![' '; h]; w];
    for i in 0..w {
        for j in 0..h {
            s_board[i][j] = s[j][i];
            t_board[i][j] = t[j][i];
        }
    }
    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();
    for i in 0..w {
            let s_key = s_board[i].iter().collect::<String>();
            let t_key = t_board[i].iter().collect::<String>();
            *s_map.entry(s_key).or_insert(0_u64) += 1;
            *t_map.entry(t_key).or_insert(0_u64) += 1;
    }
    for (key, val) in s_map.iter() {
        if let Some(x) = t_map.get(key) {
            if val != x {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

