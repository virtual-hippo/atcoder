use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    }

    let mut b = b;

    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '.' || b[i][j] == '#' {
                continue;
            }

            let x = (b[i][j] as u8 - b'0') as usize;
            b[i][j] = '.';

            for ii in 0..=x {
                for jj in 0..=x {
                    if ii + jj > x {
                        continue;
                    }
                    if i + ii < r && j + jj < c {
                        if b[i + ii][j + jj] == '#' {
                            b[i + ii][j + jj] = '.';
                        }
                    }
                    if i >= ii && j + jj < c {
                        if b[i - ii][j + jj] == '#' {
                            b[i - ii][j + jj] = '.';
                        }
                    }
                    if i + ii < r && j >= jj {
                        if b[i + ii][j - jj] == '#' {
                            b[i + ii][j - jj] = '.';
                        }
                    }
                    if i >= ii && j >= jj {
                        if b[i - ii][j - jj] == '#' {
                            b[i - ii][j - jj] = '.';
                        }
                    }
                }
            }
        }
    }

    for i in 0..r {
        for j in 0..c {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
