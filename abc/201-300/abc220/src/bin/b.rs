use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: usize,
        a: Chars,
        b: Chars,
    }
    let a_num = {
        let mut ret = 0;
        for i in 0..a.len() {
            ret += a[i].to_digit(10).unwrap() as u64 * (k.pow((a.len()-1 - i) as u32) as u64);
        }
        ret
    };
    let b_num = {
        let mut ret = 0;
        for i in 0..b.len() {
            ret += b[i].to_digit(10).unwrap() as u64 * (k.pow((b.len()-1 - i) as u32) as u64);
        }
        ret
    };
    println!("{}", a_num * b_num);
}

