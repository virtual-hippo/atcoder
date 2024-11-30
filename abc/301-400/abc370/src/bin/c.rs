use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    let mut x = vec![];
    while s != t {
        let mut f = false;
        for i in 0..t.len() {
            if s[i] > t[i] {
                s[i] = t[i];
                x.push(s.clone());
                f = true;
                break;
            }
        }
        if f {
            continue;
        }
        for i in 0..t.len() {
            let ind = t.len() - 1 - i;
            if s[ind] != t[ind] {
                s[ind] = t[ind];
                x.push(s.clone());
                break;
            }
        }
    }
    println!("{}", x.len());
    for i in 0..x.len() {
        for j in 0..x[i].len() {
            print!("{}", x[i][j]);
        }
        println!();
    }
}
