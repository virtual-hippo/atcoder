// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10],
    }

    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                let a = i;
                let b = j;
                let c = {
                    let mut current = a;
                    while current + 1 < 10 && s[current + 1][b] == '#' {
                        current += 1;
                    }
                    current
                };
                let d = {
                    let mut current = b;
                    while current + 1 < 10 && s[a][current + 1] == '#' {
                        current += 1;
                    }
                    current
                };
                
                println!("{} {}", a + 1, c + 1);
                println!("{} {}", b + 1, d + 1);
                return;
            }
        }
    }
}

