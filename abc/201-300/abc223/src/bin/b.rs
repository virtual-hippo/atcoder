use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut koho = Vec::new();
    for i in 0..s.len() {
        let current = (0..s.len())
            .map(|j| s[(j + i) % s.len()])
            .collect::<String>();
        koho.push(current)
    }
    koho.sort();
    println!("{}", koho[0]);
    println!("{}", koho[s.len() - 1]);
}
