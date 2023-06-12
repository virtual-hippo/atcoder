// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use std::collections::HashSet;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    let a = vec!['H', 'D', 'C', 'S'];
    let b = vec!['A','2','3','4','5','6','7','8','9','T','J','Q','K'];
    for _ in 0..n {
        input! {
            s: Chars,
        }
        let mut a1 = false;
        for ch in a.iter() {
            if s[0] == *ch {
                a1 = true;
            }
        }
        let mut b1 = false;
        for ch in b.iter() {
            if s[1] == *ch {
                b1 = true;
            }
        }
        if a1 && b1 && (set.contains(&(s[0], s[1])) == false){
            set.insert((s[0], s[1]));
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

