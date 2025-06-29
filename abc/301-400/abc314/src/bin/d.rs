use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    }

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(i);
    }

    let mut is_lower = 0;

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            c: char,
        }

        match t {
            1 => {
                let idx = x - 1;
                s[idx] = c;
                set.insert(idx);
            },
            2 => {
                is_lower = 1;
                set = HashSet::new();
            },
            3 => {
                is_lower = 2;
                set = HashSet::new();
            },
            _ => unreachable!(),
        }
    }

    let ans = if is_lower == 1 {
        s.iter()
            .enumerate()
            .map(|(i, &c)| if set.contains(&i) { c } else { c.to_ascii_lowercase() })
            .collect::<String>()
    } else if is_lower == 2 {
        s.iter()
            .enumerate()
            .map(|(i, &c)| if set.contains(&i) { c } else { c.to_ascii_uppercase() })
            .collect::<String>()
    } else {
        s.iter().map(|&c| c).collect::<String>()
    };

    println!("{}", ans);
}
