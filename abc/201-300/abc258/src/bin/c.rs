use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut cur = 0;
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    x: usize,
                }
                cur += n;
                cur -= x;
                cur %= n;
            }
            2 => {
                input! {
                    x: usize,
                }
                println!("{}", s[(cur + x - 1) % n]);
            }
            _ => unreachable!(),
        }
    }
}
