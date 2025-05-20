use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w - 4 {
            if s[i][j] == 's' && s[i][j + 1] == 'n' && s[i][j + 2] == 'u' && s[i][j + 3] == 'k' && s[i][j + 4] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i + 1, j + 2);
                println!("{} {}", i + 1, j + 3);
                println!("{} {}", i + 1, j + 4);
                println!("{} {}", i + 1, j + 5);
                return;
            }
            if s[i][j] == 'e' && s[i][j + 1] == 'k' && s[i][j + 2] == 'u' && s[i][j + 3] == 'n' && s[i][j + 4] == 's' {
                println!("{} {}", i + 1, j + 5);
                println!("{} {}", i + 1, j + 4);
                println!("{} {}", i + 1, j + 3);
                println!("{} {}", i + 1, j + 2);
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }

    for j in 0..w {
        for i in 0..h - 4 {
            if s[i][j] == 's' && s[i + 1][j] == 'n' && s[i + 2][j] == 'u' && s[i + 3][j] == 'k' && s[i + 4][j] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i + 2, j + 1);
                println!("{} {}", i + 3, j + 1);
                println!("{} {}", i + 4, j + 1);
                println!("{} {}", i + 5, j + 1);
                return;
            }
            if s[i][j] == 'e' && s[i + 1][j] == 'k' && s[i + 2][j] == 'u' && s[i + 3][j] == 'n' && s[i + 4][j] == 's' {
                println!("{} {}", i + 5, j + 1);
                println!("{} {}", i + 4, j + 1);
                println!("{} {}", i + 3, j + 1);
                println!("{} {}", i + 2, j + 1);
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }

    for i in 0..h - 4 {
        for j in 0..w - 4 {
            if s[i][j] == 's' && s[i + 1][j + 1] == 'n' && s[i + 2][j + 2] == 'u' && s[i + 3][j + 3] == 'k' && s[i + 4][j + 4] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i + 2, j + 2);
                println!("{} {}", i + 3, j + 3);
                println!("{} {}", i + 4, j + 4);
                println!("{} {}", i + 5, j + 5);
                return;
            }
            if s[i][j] == 'e' && s[i + 1][j + 1] == 'k' && s[i + 2][j + 2] == 'u' && s[i + 3][j + 3] == 'n' && s[i + 4][j + 4] == 's' {
                println!("{} {}", i + 5, j + 5);
                println!("{} {}", i + 4, j + 4);
                println!("{} {}", i + 3, j + 3);
                println!("{} {}", i + 2, j + 2);
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }

    for i in (4..h).rev() {
        for j in 0..w - 4 {
            if s[i][j] == 's' && s[i - 1][j + 1] == 'n' && s[i - 2][j + 2] == 'u' && s[i - 3][j + 3] == 'k' && s[i - 4][j + 4] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i, j + 2);
                println!("{} {}", i - 1, j + 3);
                println!("{} {}", i - 2, j + 4);
                println!("{} {}", i - 3, j + 5);
                return;
            }
            if s[i][j] == 'e' && s[i - 1][j + 1] == 'k' && s[i - 2][j + 2] == 'u' && s[i - 3][j + 3] == 'n' && s[i - 4][j + 4] == 's' {
                println!("{} {}", i - 3, j + 5);
                println!("{} {}", i - 2, j + 4);
                println!("{} {}", i - 1, j + 3);
                println!("{} {}", i - 0, j + 2);
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
