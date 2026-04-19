use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Chars,
    }
    let r = r + 1;

    let ans: usize = (0..n)
        .scan(vec![0; 26], |counts, i| {
            if i >= l {
                let j = (s[i - l] as u8 - b'a') as usize;
                counts[j] += 1;
            }
            if i >= r {
                let j = (s[i - r] as u8 - b'a') as usize;
                counts[j] -= 1;
            }
            let j = (s[i] as u8 - b'a') as usize;
            Some(counts[j])
        })
        .sum();

    println!("{}", ans);
}
