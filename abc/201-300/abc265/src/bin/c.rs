use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        (h,w): (usize, usize),
        g: [Chars; h],
    }
    let mut set = HashSet::new();
    let mut curr = (1,1);
    set.insert(curr);
    loop {
        match g[curr.0-1][curr.1-1] {
            'U' => {
                if curr.0 != 1 {
                    curr.0 -= 1;
                } else {
                    println!("{} {}", curr.0, curr.1);
                    return;
                }
            }
            'D' => {
                if curr.0 != h {
                    curr.0 += 1;
                } else {
                    println!("{} {}", curr.0, curr.1);
                    return;
                }
            }
            'L' => {
                if curr.1 != 1 {
                    curr.1 -= 1;
                } else {
                    println!("{} {}", curr.0, curr.1);
                    return;
                }
            }
            'R' => {
                if curr.1 != w {
                    curr.1 += 1;
                } else {
                    println!("{} {}", curr.0, curr.1);
                    return;
                }
            }
            _ => unreachable!()
        }
        if set.contains(&curr) {
            println!("-1");
            return;
        } else {
            set.insert(curr);
        }
    }
}

