// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }
    let set_o = s.iter().enumerate().filter(|(_, &ch)| ch == 'o').map(|(i,_)| i).collect::<HashSet<usize>>();
    let set_x = s.iter().enumerate().filter(|(_, &ch)| ch == 'x').map(|(i,_)| i).collect::<HashSet<usize>>();
    
    if set_o.len() > 4 || set_x.len() == 10 {
        println!("0");
        return;
    }
    let mut ans = 0;
    for i in 0..10000 {
        let current = format!("{:>04}", i);
        let is_ok1 = {
            let mut res = true;
            for val in set_o.iter() {
                let val_str = format!("{}", val);
                if current.contains(&val_str) == false {
                    res = false;
                    break;
                }
            }
            res
        };
        let is_ok2 = {
            let mut res = true;
            for val in set_x.iter() {
                let val_str = format!("{}", val);
                if current.contains(&val_str) {
                    res = false;
                    break;
                }
            }
            res
        };
        if is_ok1 && is_ok2 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

