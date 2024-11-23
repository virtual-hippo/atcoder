use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ret: Vec<usize> = vec![];

    let mut ans: usize = 1;

    for i in 0..n {
        if s[i] == '1' {
            if ret.len() == 0 {
                ret.push(1);
            } else if ret.len() == 1 {
                ret[0] += 1;
            } else if ret.len() == 3 {
                ans = ans.max(ret[0].min(ret[2]) * 2 + 1);
                ret.clear();
                ret.push(1);
            } else if ret.len() == 2 {
                ret.clear();
                ret.push(1);
            }
        } else if s[i] == '/' {
            if ret.len() == 1 {
                ret.push(1);
            } else if ret.len() == 3 {
                ans = ans.max(ret[0].min(ret[2]) * 2 + 1);
                ret.clear();
            } else if ret.len() == 2 {
                ret.clear();
            }
        } else if s[i] == '2' {
            if ret.len() == 2 {
                ret.push(1);
                ans = ans.max(ret[0].min(ret[2]) * 2 + 1);
            } else if ret.len() == 3 {
                ret[2] += 1;
                ans = ans.max(ret[0].min(ret[2]) * 2 + 1);
            } else if ret.len() == 1 {
                ret.clear();
            }
        }
    }
    if ret.len() == 3 {
        ans = ans.max(ret[0].min(ret[2]) * 2 + 1);
    }
    println!("{}", ans);
}
