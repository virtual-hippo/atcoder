use proconio::{fastout, input};

fn is_kaibun(s: &Vec<char>) -> bool {
    let s_len = s.len();
    for i in 0..s_len / 2 {
        if s[i] != s[s_len - 1 - i] {
            return false;
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut i = 1;
    let mut ans = 1;
    while i * i * i <= n {
        let str_val = format!("{}", i * i * i);
        if is_kaibun(&(str_val.chars().collect::<Vec<char>>())) {
            ans = i * i * i;
        }
        i += 1;
    }
    println!("{}", ans);
}
