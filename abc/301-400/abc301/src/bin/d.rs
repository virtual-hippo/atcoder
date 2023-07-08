use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        n: u64,
    }
    let n_vec = format!("{:>060b}", n).chars().collect::<Vec<char>>();
    let s_vec = (0..60)
        .map(|i| {
            if i >= 60 - s.len() {
                s[i - (60 - s.len())]
            } else {
                '0'
            }
        })
        .collect::<Vec<char>>();


    let mut max = 0;
    for i in 0..60 {
        if n_vec[i] == '1' && s_vec[i] == '?' {
            max = i;
        }
    }
    let is_last_change = (max..60).filter(|&i| n_vec[i] == '0' && s_vec[i] == '1').count() > 0;
    
    let mut is_less = false;
    let mut ans = s_vec.clone();
    for i in 0..60 {
        if !is_less && n_vec[i] == '0' && s_vec[i] == '1' {
            println!("-1");
            return;
        }
        if !is_less && n_vec[i] == '1' && s_vec[i] == '0' {
            is_less = true;
        }

        if s_vec[i] == '?' {
            if is_less {
                ans[i] = '1';
            } else {
                if i == max && is_last_change {
                    ans[i] = '0';
                    is_less = true;
                } else {
                    ans[i] = n_vec[i];
                }
            }
        }
    }
    let ans_string: String = ans.iter().collect();
    let ans_u64 = u64::from_str_radix(ans_string.as_str(), 2).unwrap();
    if ans_u64 <= n as u64 {
        println!("{}", ans_u64);
    } else {
        println!("-1");
    }
}

