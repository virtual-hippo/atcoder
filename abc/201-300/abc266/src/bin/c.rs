// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn gaiseki(a: (i64, i64), v: (i64, i64)) -> i64 {
    a.0 * v.1 - a.1 * v.0
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
        d: (i64, i64),
    }
    let ab = ((b.0 - a.0), (b.1 - a.1));
    let bc = ((c.0 - b.0), (c.1 - b.1));
    let cd = ((d.0 - c.0), (d.1 - c.1));
    let da = ((a.0 - d.0), (a.1 - d.1));
    if gaiseki(ab, bc) > 0 && gaiseki(bc, cd) > 0 && gaiseki(cd, da) > 0 && gaiseki(da, ab) > 0 {
        println!("Yes");
        return;
    } else {
        println!("No");
    }
}
