// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

struct Waza {
    t: usize,
    a: Vec<usize>,
}

fn exec(ans: &mut u64, waza_vec: &Vec<Waza>, learned: &mut Vec<bool>, pos: usize) {
    learned[pos] = true;
    *ans += waza_vec[pos].t as u64;
    for point in waza_vec[pos].a.iter() {
        if learned[*point] == false {
            exec(ans, waza_vec, learned, *point);
        }
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut waza = Vec::with_capacity(n + 1);
    waza.push(Waza { t: 0, a: vec![0] });
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            a: [usize; k],
        }
        waza.push(Waza { t, a });
    }
    let mut ans = 0;
    let mut learned = vec![false; n + 1];
    exec(&mut ans, &waza, &mut learned, n);
    println!("{}", ans);
}
