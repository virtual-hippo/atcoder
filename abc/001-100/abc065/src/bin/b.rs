use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut next = 1;
    let mut cnt = 0;
    let mut visited = vec![false; n];
    while visited[next-1] == false {
        visited[next-1] = true;
        next = a[next-1];
        cnt += 1;
        if next == 2 {
            println!("{}", cnt);
            return;
        }
    }
    println!("{}", -1);
}

