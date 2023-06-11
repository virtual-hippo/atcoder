use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut head = 0;
    let mut tail = s.len()-1;
    while head < tail {
        if s[tail] == s[head] {
            tail -= 1;
            head += 1;
        } else if s[tail] == 'a' {
            tail -= 1;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

