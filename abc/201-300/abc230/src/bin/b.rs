// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    if s.len() == 1 {
        println!("Yes");
    } else if s.len() == 2 {
        if s[0] == 'o' && s[1] == 'o' {
            println!("No");
        } else {
            println!("Yes");
        }
    } else if s[0] == 'o' {
        for i in 0..s.len() {
            if i % 3 == 0 &&  s[i] != 'o' {
                println!("No");
                return;
            }
            if i % 3 > 0 &&  s[i] != 'x' {
                println!("No");
                return;
            }
        }
        println!("Yes");
    } else if s[0] == 'x' && s[1] == 'o' {
        for i in 0..s.len() {
            if i % 3 == 1 && s[i] != 'o' {
                println!("No");
                return;
            }
            if (i % 3 == 0 || i % 3 == 2) &&  s[i] != 'x' {
                println!("No");
                return;
            }
        }
        println!("Yes");
    } else if s[0] == 'x' && s[1] == 'x' && s[2] == 'o' {
        for i in 0..s.len() {
            if i % 3 == 2 &&  s[i] != 'o' {
                println!("No");
                return;
            }
            if i % 3 < 2 && s[i] != 'x' {
                println!("No");
                return;
            }
        }
        println!("Yes");
    } else {
        println!("No");
    }
}

