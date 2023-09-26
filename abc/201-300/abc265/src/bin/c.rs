use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        (h,w): (usize, usize),
        g: [Chars; h],
    }
    let mut seen = HashSet::new();
    let mut pos = (0, 0);
    while seen.contains(&pos) == false {
        seen.insert(pos);
        if g[pos.0][pos.1] == 'U' && pos.0 != 0 {
            pos.0 -= 1;
        } else if g[pos.0][pos.1] == 'D' && pos.0 != h - 1 {
            pos.0 += 1;
        } else if g[pos.0][pos.1] == 'L' && pos.1 != 0 {
            pos.1 -= 1;
        } else if g[pos.0][pos.1] == 'R' && pos.1 != w - 1 {
            pos.1 += 1;
        } else {
            println!("{} {}", pos.0 + 1, pos.1 + 1);
            return;
        }
    }
    println!("{}", -1);
}
