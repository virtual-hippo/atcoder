use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = 1;
    let mut i = 0;
    while i < n {
        if s[i] == '/' {
            let mut l = i;
            let mut r = i;
            let mut cnt = 1;

            while l > 0 && r < n - 1 {
                if s[l - 1] == '1' && s[r + 1] == '2' {
                    cnt += 2;
                    l -= 1;
                    r += 1;
                    ans = ans.max(cnt);
                } else {
                    i = r;
                    break;
                }
            }
        }
        i += 1;
    }

    println!("{}", ans);
}
