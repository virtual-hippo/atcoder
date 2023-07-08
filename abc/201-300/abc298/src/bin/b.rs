use proconio::input;

fn is_ok(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> bool {
    for i in 0..a.len() {
        for j in 0..a.len() {
            if a[i][j] == 1 && b[i][j] == 0 {
                return false;
            }
        }
    }
    true
}

fn convert(table: &mut Vec<Vec<usize>>) {
    let clone = table.clone();
    let n = table.len();
    for i in 0..n {
        for j in 0..n {
            table[i][j] = clone[(n-1)-j][i];
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    let mut current = a;
    for _ in 0..4 {
        if is_ok(&current, &b) {
            println!("Yes");
            return;
        }
        convert(&mut current);
    }
    println!("No");
}

