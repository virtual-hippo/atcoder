// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    for i in 0..m {
        if let Some(val) = map.get_mut(&b[i]) {
            if *val > 0 {
                *val -= 1;
            } else {
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

