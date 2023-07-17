use proconio::input;
use proconio::marker::Chars;

fn is_large(s: &Vec<char>, n: u64) -> bool {
    let s_as_string = s.iter().map(|&ch| ch).collect::<String>();
    let s_as_num = u64::from_str_radix(s_as_string.as_str(), 2).unwrap();
    s_as_num > n
}

fn main() {
    input! {
        s: Chars,
        n: u64,
    }
    let mut current = s.iter().map(|&ch| if ch == '?' {'0'} else {ch}).collect::<Vec<char>>();
    if is_large(&current, n) {
        println!("{}", -1);
        return;
    }

    for i in 0..s.len() {
        if s[i] == '?' {
            current[i] = '1';
            if is_large(&current, n) {
                current[i] = '0';
            }
        }
    }

    let current_as_string = current.iter().map(|&ch| ch).collect::<String>();
    println!("{}", u64::from_str_radix(current_as_string.as_str(), 2).unwrap());
}
