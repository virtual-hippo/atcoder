use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: i64,
        k: i64,
        s: Chars,
    }
    let mut set = HashSet::with_capacity(m);
    for _ in 0..m {
        input! {
            x: i64,
            y: i64,
        }
        set.insert((x,y));
    }

    let mut current = (0,0);
    let mut tairyoku = h;

    for i in 0..n {
        if tairyoku == 0 {
            println!("No");
            return;
        }
        tairyoku -= 1;
        if s[i] == 'R' {
            current.0 += 1;
        } else if s[i] == 'L' {
            current.0 -= 1;
        } else if s[i] == 'U' {
            current.1 += 1;
        } else if s[i] == 'D' {
            current.1 -= 1;
        }

        if set.contains(&current) {
            if tairyoku < k {
                set.remove(&current);
                tairyoku = k;
            }
        }
    }
    
    println!("Yes");
}

