// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let s = format!("{:b}", n);
    let ch_vec = s.to_string().chars().collect::<Vec<char>>();
    let len = ch_vec.len();

    let mut one_pos = vec![];
    for i in 0..len {
        if ch_vec[i] == '1' {
            one_pos.push(i);
        }
    }

    let mut answers = vec![];
    for bit in 0..(1<<one_pos.len()) {
        let mut current = vec![false; one_pos.len()];
        for i in 0..one_pos.len() {
            if bit & (1 << i) == 0 {
                current[i] = true;
            }
        }
        let mut ans = ch_vec.clone();
        for i in 0..ch_vec.len() {
            for j in 0..one_pos.len() {
                if i == one_pos[j] {
                    if current[j] {
                        ans[i] = '1';
                    } else {
                        ans[i] = '0';
                    }
                }
            }
        }
        answers.push(ans.iter().collect::<String>());
    }
    answers.sort();
    for answer in answers {
        println!("{}", u64::from_str_radix(&answer, 2).unwrap());
    }


}

