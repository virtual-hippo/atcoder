use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for w in 1..s.len() {
        let mut vals = vec![];
        for i in (0..s.len()).step_by(w) {
            vals.push(s[i..(i + w).min(s.len())].iter().copied().collect::<Vec<char>>());
        }

        for i in 0..w {
            let s = vals.iter().filter(|v| v.len() > i).map(|v| v[i]).collect::<Vec<char>>();

            if s == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
