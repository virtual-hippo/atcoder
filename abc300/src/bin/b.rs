use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (h,w): (usize, usize),
        a: [Chars; h],
        b: [Chars; h],
    }
    let check = |x: Vec<Vec<char>>| {
        for i in 0..h {
            for j in 0..w {
                if x[i][j] != b[i][j] {
                    return false;
                }
            }
        }
        true
    };

    for s in 0..h {
        let mut current = vec![];
        for i in s..h {
            current.push(a[i].clone());
        }
        for i in 0..s {
            current.push(a[i].clone());
        }
        for t in 0..w {
            let mut current2 = vec![vec![]; h];
            for i in 0..h {
                for j in t..w {
                    current2[i].push(current[i][j]);
                }
                for j in 0..t {
                    current2[i].push(current[i][j]);
                }
            }
            if check(current2) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
