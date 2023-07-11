use proconio::input;

fn slove(
    vec: &mut Vec<usize>,
    n: &usize,
    m: &usize,
    val: &usize,
    abcd: &Vec<(usize, usize, usize, usize)>,
) -> usize {
    let mut max = 0;
    if vec.len() == *n {
        return abcd
            .iter()
            .filter(|&&(a, b, c, _)| vec[b - 1] - vec[a - 1] == c)
            .fold(0, |sum, &(_, _, _, d)| sum + d);
    }
    for v in *val..*m + 1 {
        vec.push(v);
        max = std::cmp::max(slove(vec, n, m, &v, abcd), max);
        vec.pop();
    }
    max
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(usize,usize,usize,usize); q],
    }
    let mut vec: Vec<usize> = Vec::with_capacity(n);
    let ans = slove(&mut vec, &n, &m, &1, &abcd);
    println!("{}", ans);
}
