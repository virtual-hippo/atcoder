// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut cnt = 0;
    let mut vec = vec![];
    for i in 0..n-1 {
        if a[i] > a[i+1] {
            cnt += 1;
            vec.push(i);
        }
        if cnt > 2 {
            println!("NO");
            return;
        }
    }
    if cnt == 0 {
        println!("YES");
        return;
    }
    if cnt == 1 {
        println!("NO");
        return;
    }
    let tmp = a[vec[0]];
    a[vec[0]] = a[vec[1]+1];
    a[vec[1]+1] = tmp;
    for i in 0..n-1 {
        if a[i] > a[i+1] {
            println!("NO");
            return;
        }
    }

    println!("YES");
}

