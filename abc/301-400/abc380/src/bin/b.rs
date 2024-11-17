use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut ans = vec![];
    let mut cnt = 0;
    for ch in s.chars() {
        if ch == '|' {
            if cnt != 0 {
                ans.push(cnt);
            }
            cnt = 0;
        } else {
            cnt += 1;
        }
    }
    for v in ans.iter() {
        print!("{} ", v);
    }
}
