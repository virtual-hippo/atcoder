use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let f = |c1, c2| {
        c1 == c2
            || (c1 == '1' && c2 == 'l')
            || (c1 == 'l' && c2 == '1')
            || (c1 == 'o' && c2 == '0')
            || (c1 == '0' && c2 == 'o')
    };
    for i in 0..n {
        if f(s[i], t[i]) == false {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
