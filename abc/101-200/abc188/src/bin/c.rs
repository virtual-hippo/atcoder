use proconio::input;

fn solve(a: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if a.len() == 2 {
        return a.clone();
    }
    let mut current = Vec::with_capacity(a.len()/2);
    for i in 0..a.len()/2 {
        if a[2 * i].1 > a[2 * i + 1].1 {
            current.push(a[2 * i]);
        } else {
            current.push(a[2 * i + 1]);
        }
    }
    solve(&current)
}

fn main() {
    input! {
        n: usize,
    }
    let len = 2_usize.pow(n as u32);
    input! {
        a: [usize; len],
    }
    let a_enumerate = a.iter().enumerate().map(|(i, &v)|(i, v)).collect::<Vec<(usize,usize)>>();
    let ret = solve(&a_enumerate);
    let ans = if ret[0].1 < ret[1].1 {
        ret[0].0 + 1
    } else {
        ret[1].0 + 1
    };
    println!("{}", ans);
}

