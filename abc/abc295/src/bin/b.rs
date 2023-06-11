// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (r,c): (usize, usize),
        mut b: [Chars; r],
    }
    let mut breaked = vec![vec![false; c]; r];

    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '.' || b [i][j] == '#' {
                continue;
            }
            let bomb: usize = b [i][j] as usize - 48;
            for k in 0..r {
                for l in 0..c {
                    let y = if k <= i {
                        i - k
                    } else {
                        k - i
                    };
                    let x = if l <= j {
                        j - l
                    } else {
                        l - j
                    };
                    let manh = x + y;
                    if manh <= bomb {
                        breaked[k][l] = true;
                    }
                }
            }
        }
    }
    
    for i in 0..r {
        for j in 0..c {
            if breaked[i][j] {
                b[i][j] = '.';
            }
        }
    }

    for i in 0..r {
        for j in 0..c {
            print!("{}", b[i][j]);
        }
        println!("");
    }
}

