use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [Chars; h],
    }
    let f = |i: usize, j: usize| {
        if a[i][j] == '+' {
            1
        } else {
            -1
        }
    };
    
    let mut opt = vec![vec![0; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h-1 && j == w-1 {
                continue;
            } else if i == h-1 {
                if (i + j) % 2 == 0 {
                    opt[i][j] = opt[i][j+1] + f(i, j+1);
                } else {
                    opt[i][j] = opt[i][j+1] - f(i, j+1);
                }
            } else if j == w-1 {
                if (i + j) % 2 == 0 {
                    opt[i][j] = opt[i+1][j] + f(i+1, j);
                } else {
                    opt[i][j] = opt[i+1][j] - f(i+1, j);
                }
            } else {
                if (i + j) % 2 == 0 {
                    opt[i][j] = std::cmp::max(opt[i+1][j] + f(i+1, j), opt[i][j+1] + f(i, j+1));
                } else {
                    opt[i][j] = std::cmp::min(opt[i+1][j] - f(i+1, j), opt[i][j+1] - f(i, j+1));
                }
            }
            
        }
    }
    if opt[0][0] > 0 {
        println!("Takahashi");
    } else if opt[0][0] == 0 {
        println!("Draw");
    } else {
        println!("Aoki");
    }
}
