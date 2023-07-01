use proconio::input;

fn get_a(n: &usize) -> Vec<Vec<usize>> {
    let mut a = vec![vec![0_usize; 2*n]; 2*n];
    for i in 0..2*n-1 {
        for j in i+1..2*n {
            input! {
                aa: usize,
            }
            a[i][j] = aa;
        }
    }
    a
}

fn solve(n: &usize, a: &Vec<Vec<usize>>,pairs: &mut Vec<(usize,usize)>, used: &mut Vec<bool>) -> usize {
    let mut ret = 0;
    if pairs.len() == *n {
        return pairs.iter().map(|&(i,j)|a[i][j]).fold(0, |ret, x| ret ^ x);
    }
    let left = used.iter().enumerate().find(|(_, &state)| state == false).unwrap().0;
    used[left] = true;
    for i in 0..used.len() {
        if used[i] == false {
            used[i] = true;
            pairs.push((left, i));
            ret = std::cmp::max(ret, solve(n, a, pairs, used));
            used[i] = false;
            pairs.pop();
        }
    }
    used[left] = false;
    ret
}

fn main() {
    input! {
        n: usize,
    }
    let a = get_a(&n);
    let mut pairs = Vec::with_capacity(n);
    let mut used = vec![false; 2*n];

    println!("{}", solve(&n, &a, &mut pairs, &mut used));
}

