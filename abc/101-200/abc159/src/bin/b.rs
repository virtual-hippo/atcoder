// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn is_kaibun(s: &Vec<char>) -> bool{
    let s_len = s.len();
    for i in 0..s_len/2 {
        if s[i] != s[s_len-1-i] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        s: Chars,
    }
    let a = is_kaibun(&s);
    let s1 = s[0..(s.len()-1)/2].iter().map(|&ch| ch).collect();
    let b = is_kaibun(&s1);
    let s2 = s[((s.len()+3)/2-1)..s.len()].iter().map(|&ch| ch).collect();
    let c = is_kaibun(&s2);
    if a && b && c {
        println!("Yes");
    } else {
        println!("No");
    }
}

