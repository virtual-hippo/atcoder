use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut c = 0;
    let mut ch = 0;
    let mut cho = 0;
    let mut chok = 0;
    let mut choku = 0;
    let mut chokud = 0;
    let mut chokuda = 0;
    let mut chokudai: u64 = 0;
    for i in 0..s.len() {
        if s[i] == 'c' {
            c += 1;
            c %= 1_000_000_007;
        } else if  s[i] == 'h' {
            ch += c;
            ch %= 1_000_000_007;
        }  else if  s[i] == 'o' {
            cho += ch;
            cho %= 1_000_000_007;
        } else if  s[i] == 'k' {
            chok += cho;
            chok %= 1_000_000_007;
        } else if  s[i] == 'u' {
            choku += chok;
            choku %= 1_000_000_007;
        } else if  s[i] == 'd' {
            chokud += choku;
            chokud %= 1_000_000_007;
        } else if  s[i] == 'a' {
            chokuda += chokud;
            chokuda %= 1_000_000_007;
        } else if  s[i] == 'i' {
            chokudai += chokuda;
            chokudai %= 1_000_000_007;
        }
        chokudai %= 1_000_000_007;
    }

    println!("{}", chokudai);
}

