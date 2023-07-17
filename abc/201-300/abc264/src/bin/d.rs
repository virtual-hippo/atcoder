use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let atcoder = vec!['a','t','c','o','d','e','r'];
    let mut map = HashMap::new();
    let mut que = VecDeque::new();
    map.insert(s.clone(), 0);
    que.push_back(s);
    while que.is_empty() == false {
        let current = que.pop_front().unwrap();
        if current == atcoder {
            println!("{}", map.get(&atcoder).unwrap());
            return;
        }

        for i in 1..7 {
            let mut next = current.clone();
            let tmp = next[i-1];
            next[i-1] = next[i];
            next[i] = tmp;
            
            if map.contains_key(&next) == false {
                que.push_back(next.clone());
                map.insert(next, map.get(&current).unwrap()+1);
            }
        }
    }
}

