// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

struct Pos {
    length: usize,
    left: usize,
    right: usize,
}

impl Pos {
    fn new() -> Self {
        Pos {
            length: 0,
            left: (0, 0),
            right: (0, 0),
        }
    }
}

fn main() {
    input! {
        (h,w): (usize, usize),
        s: [Chars; h],
    }
    let max_width_pos = {
        let mut res = Pos::new();
        for i in 0..h {
            let mut current = Pos::new();
            let mut vec = vec![];
            let mut is_block = false;
            for j in 0..w {
                if s[i][j] == '.' {
                    if is_block {
                        is_block = false;
                        current.left = (i, j);
                    }
                    current.length += 1;
                } else {
                    is_block = true;
                    current.right = (i, j);
                    vec.push(current);
                    current = Pos::new();
                }
                if j == w-1 {
                    current.right = (i, j);
                    vec.push(current);
                }
            }
            res = std::cmp::max(res, vec.iter());
        }
    };

    let max_height_pos = {
        for i in 0..h {
            for j in 0..w {
                //
            }
        }
    };
}

