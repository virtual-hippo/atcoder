use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        m: usize,
        s: String,
    }
    let mut muji = m;
    let mut logo_zan = 0;
    let mut logo = 0;
    for ch in s.chars() {
        if ch == '0' {
            muji = m;
            logo_zan = logo;
        } else if ch == '1' {
            if muji > 0 {
                muji -= 1;
            } else if logo_zan > 0 {
                logo_zan -= 1;
            } else {
                logo += 1;
            }
        } else if ch == '2' {
            if logo_zan > 0 {
                logo_zan -= 1;
            } else {
                logo += 1;
            }
        }
    }
    println!("{}", logo);
}
