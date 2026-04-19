use proconio::{input, marker::*};

fn convert(s: &Vec<char>) -> Vec<char> {
    let n = s.len();
    let mut c = vec![];
    for i in 0..n {
        match s[i] {
            '(' => {
                c.push(s[i]);
            },
            'x' => {
                c.push(s[i]);
            },
            ')' => {
                if c.len() >= 3 && c[c.len() - 1] == 'x' && c[c.len() - 2] == 'x' && c[c.len() - 3] == '(' {
                    c.pop();
                    c.pop();
                    c.pop();
                    c.push('x');
                    c.push('x');
                } else {
                    c.push(s[i]);
                }
            },
            _ => unreachable!(),
        }
    }
    c
}

fn solve() {
    input! {
        a: Chars,
        b: Chars,
    }

    let a = convert(&a);
    let b = convert(&b);
    if a == b {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}
