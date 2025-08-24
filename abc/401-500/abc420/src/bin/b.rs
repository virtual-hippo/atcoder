use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut tokuten = vec![0; n];

    for j in 0..m {
        let mut cnt_0 = 0;
        let mut cnt_1 = 0;
        for i in 0..n {
            if s[i][j] == '0' {
                cnt_0 += 1;
            } else {
                cnt_1 += 1;
            }
        }

        if cnt_0 == 0 || cnt_1 == 0 {
            for i in 0..n {
                tokuten[i] += 1;
            }
        } else {
            let ch = if cnt_0 < cnt_1 { '0' } else { '1' };
            for i in 0..n {
                if s[i][j] == ch {
                    tokuten[i] += 1;
                }
            }
        }
    }

    let mx = *tokuten.iter().max().unwrap();

    for i in 0..n {
        if tokuten[i] == mx {
            print!("{} ", i + 1);
        }
    }
}
