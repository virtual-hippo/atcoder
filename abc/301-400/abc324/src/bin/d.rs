use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }
    s.sort();

    let mut ans = 0_u64;
    let mut i = 0;
    while i * i < 10_u64.pow(n as u32) {
        let mut cur = (i * i).to_string().chars().collect::<Vec<char>>();
        while cur.len() < s.len() {
            cur.push('0');
        }
        cur.sort();
        if s == cur {
            ans += 1;
        }
        i += 1;
    }
    println!("{ans}")
}
