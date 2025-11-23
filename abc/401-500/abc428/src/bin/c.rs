use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut s = vec![0];
    let mut t = vec![0];

    for _ in 0..q {
        input! {
            q: usize,
        }
        match q {
            1 => {
                input! {
                    c: char,
                }

                let v = s[s.len() - 1] + if c == '(' { 1 } else { -1 };
                s.push(v);
                let mn = t[t.len() - 1].min(v);
                t.push(mn);
            },
            2 => {
                s.pop();
                t.pop();
            },
            _ => unreachable!(),
        }
        if s[s.len() - 1] == 0 && t[t.len() - 1] >= 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
