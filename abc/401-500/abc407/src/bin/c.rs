use proconio::{fastout, input, marker::Chars};

fn minus(v: usize, cnt: usize) -> usize {
    (v + 10 - (cnt % 10)) % 10
}

#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    let mut t = n.iter().map(|&c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();

    let mut cnt = 0;
    while t.len() > 0 {
        let tail = t[t.len() - 1];
        cnt += minus(tail, cnt);
        t.pop();
    }

    let ans = cnt + n.len();
    println!("{}", ans);
}
