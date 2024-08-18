use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut cnt = vec![0; 26];
    for ch in s.chars() {
        let i = ch as u8;
        cnt[i as usize - 97] += 1;
    }
    let mut ans = (0, 0);
    for i in 0..26 {
        if cnt[i] > ans.1 {
            ans = (i, cnt[i]);
        }
    }
    println!("{}", ((ans.0 + 97) as u8) as char);
}
