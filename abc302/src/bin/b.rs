// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;



fn main() {
    input! {
        (h,w): (usize, usize),
        s: [Chars; h],
    }
    let diff = vec![-1, 0,1];
    let snuke =  vec!['s', 'n', 'u', 'k','e'];
    let mut ans = vec![(0,0); 5];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != 's' {
                continue;
            }
            for ii in 0..3 {
                for jj in 0..3 {
                    let mut current = 0;
                    let mut x = i as i32 + current * diff[ii];
                    let mut y = j as i32 + current * diff[jj];
                    while 0 <= x && x < h as i32 && 0 <= y && y < w as i32 && current < 5 && s[x as usize][y as usize] == snuke[current as usize] {
                        ans[current as usize].0 = x;
                        ans[current as usize].1 = y;
                        current += 1;
                        x = i as i32 + current * diff[ii];
                        y = j as i32 + current * diff[jj];
                    }
                    if current == 5 {
                        for k in 0..5 {
                            println!("{} {}", ans[k].0+1,ans[k].1+1);
                        }
                    }
                }
            }
        }
    }
}

