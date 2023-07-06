use proconio::input;
use proconio::marker::Chars;

fn is_ok(s: Vec<char>) -> bool {
    for i in 0..s.len()/2 {
        if s[i] != s[s.len()-1-i] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    for i in 0..n-1 {
        for j in i+1..n {
            let mut vec1 = s[i].clone();
            let mut vec2 = s[j].clone();
            vec1.extend(s[j].clone());
            vec2.extend(s[i].clone());
            if is_ok(vec1) || is_ok(vec2) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

