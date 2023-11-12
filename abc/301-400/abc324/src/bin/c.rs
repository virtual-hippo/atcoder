use proconio::input;
use proconio::marker::Chars;

fn is_ok(s: &Vec<char>, t: &Vec<char>) -> bool {
    let mut diff = 0;
    for j in 0..s.len() {
        if s[j] != t[j + diff] {
            diff += 1;
        }
        if diff > 1 || s[j] != t[j + diff] {
            return false;
        }
    }
    true
}
fn main() {
    input! {
        n: usize,
        t: Chars,
    }
    let ans = (0..n)
        .filter_map(|i| {
            input! {
                s: Chars
            }
            if s == t {
                Some(i)
            } else if s.len() + 1 == t.len() {
                if is_ok(&s, &t) {
                    Some(i)
                } else {
                    None
                }
            } else if t.len() + 1 == s.len() {
                if is_ok(&t, &s) {
                    Some(i)
                } else {
                    None
                }
            } else if s.len() == t.len() {
                let mut cnt = 0;
                for j in 0..s.len() {
                    if s[j] != t[j] {
                        cnt += 1;
                    }
                    if cnt > 1 {
                        return None;
                    }
                }
                Some(i)
            } else {
                None
            }
        })
        .map(|i| i + 1)
        .collect::<Vec<usize>>();
    println!("{}", ans.len());
    for v in ans.iter() {
        print!("{} ", v);
    }
}
