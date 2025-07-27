use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut t = s.clone();

    let mut flag = true;
    for i in 0..s.len() {
        if t[i] == '.' && flag {
            if i > 0 && t[i - 1] == 'o' {
                continue;
            }
            t[i] = 'o';
            flag = false;
            continue;
        }
        if t[i] == '#' && !flag {
            flag = true;
            continue;
        }
    }

    let ans = t.into_iter().collect::<String>();
    println!("{}", ans);
}
