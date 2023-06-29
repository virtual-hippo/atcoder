use proconio::input;
use proconio::marker::Chars;

fn check_yoko(row: usize, s: &Vec<Vec<char>>) -> bool {
    let n = s.len();
    for i in 0..n-5 {
        let mut current = 0;
        for j in 0..6 {
            if s[row][i+j] == '#' {
                current += 1;
            }
            // print!("{}", s[row][i+j]);
        }
        if current >= 4 {
            return true;
        }
        // println!("");
    }
    false
}

fn check_tate(col: usize, s: &Vec<Vec<char>>) -> bool {
    let n = s.len();
    for i in 0..n-5 {
        let mut current = 0;
        for j in 0..6 {
            if s[i+j][col] == '#' {
                current += 1;
            }
            // print!("{}", s[row][i+j]);
        }
        if current >= 4 {
            return true;
        }
        // println!("");
    }
    false
}

fn check_migikata(row: usize, col: usize, s: &Vec<Vec<char>>) -> bool {
    let mut current = 0;
    for j in 0..6 {
        if s[row + j][col + j] == '#' {
            current += 1;
        }
        // print!("{}", s[row][i+j]);
    }
    if current >= 4 {
        return true;
    }
    false
}

fn check_hidarikata(row: usize, col: usize, s: &Vec<Vec<char>>) -> bool {
    let mut current = 0;
    for j in 0..6 {
        if s[row - j][col + j] == '#' {
            current += 1;
        }
        // print!("{}", s[row][i+j]);
    }
    if current >= 4 {
        return true;
    }
    false
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    for i in 0..n {
        if check_yoko(i, &s) {
            println!("Yes");
            return;
        }
        if check_tate(i, &s) {
            println!("Yes");
            return;
        }
    }
    for i in 0..n {
        for j in 0..n {
            if n - i > 5 && n - j > 5 {
                if check_migikata(i, j, &s) {
                    println!("Yes");
                    return;
                }
            }
            if i > 4 && n - j > 5 {
                if check_hidarikata(i, j, &s) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}

