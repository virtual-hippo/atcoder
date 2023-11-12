use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut stack = Vec::with_capacity(s.len());
    for &ch in s.iter() {
        stack.push(ch);
        if stack.len() > 0
            && stack[stack.len() - 1] == 'C'
            && stack.len() > 1
            && stack[stack.len() - 2] == 'B'
            && stack.len() > 2
            && stack[stack.len() - 3] == 'A'
        {
            stack.pop();
            stack.pop();
            stack.pop();
        }
    }
    for ch in stack.iter() {
        print!("{}", ch);
    }
}
