use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        mut t: Chars,
    }
    let len = if t[2] == 'X' {
        t.pop();
        2
    } else {
        3
    };
    let mut start = 0;
    let mut ind = vec![];
    for &ch1 in t.iter() {
        for i in start..s.len() {
            if ch1 as u8 + 32 == s[i] as u8 {
                ind.push(i);
                start = i + 1;
                break;
            }
        }
        if ind.len() == len {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
